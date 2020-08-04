import bpy
import math
import random

def clear_scene():
    bpy.data.objects.remove(bpy.data.objects['Cube'])

def setup_denoising():
    if bpy.context.scene.render.engine != 'CYCLES':
        return

    bpy.context.view_layer.cycles.denoising_store_passes = True
    bpy.context.scene.use_nodes = True
    bpy.context.scene.node_tree.nodes.clear()

    tree = bpy.context.scene.node_tree

    composite = tree.nodes.new('CompositorNodeComposite')
    composite.use_alpha = True
    render_layers = tree.nodes.new('CompositorNodeRLayers')
    denoise = tree.nodes.new('CompositorNodeDenoise')
    denoise.use_hdr = True

    tree.links.new(render_layers.outputs['Noisy Image'], denoise.inputs['Image'])
    tree.links.new(render_layers.outputs['Denoising Normal'], denoise.inputs['Normal'])
    tree.links.new(render_layers.outputs['Denoising Albedo'], denoise.inputs['Albedo'])

    tree.links.new(denoise.outputs['Image'], composite.inputs['Image'])

# Technique adapted from https://www.youtube.com/watch?v=5OUpqvx6RE8.
def setup_orb_material(mat, color):
    mat.use_nodes = True
    mat.node_tree.nodes.clear()

    light_path = mat.node_tree.nodes.new('ShaderNodeLightPath')
    minimum = mat.node_tree.nodes.new('ShaderNodeMath')
    glass = mat.node_tree.nodes.new('ShaderNodeBsdfGlass')
    translucent = mat.node_tree.nodes.new('ShaderNodeBsdfTranslucent')
    mix_shader1 = mat.node_tree.nodes.new('ShaderNodeMixShader')
    mix_shader2 = mat.node_tree.nodes.new('ShaderNodeMixShader')
    diffuse = mat.node_tree.nodes.new('ShaderNodeBsdfDiffuse')
    fresnel = mat.node_tree.nodes.new('ShaderNodeFresnel')
    output = mat.node_tree.nodes.new('ShaderNodeOutputMaterial')

    minimum.operation = 'MINIMUM'
    glass.inputs['Roughness'].default_value = 0
    glass.inputs['IOR'].default_value = 1.45
    diffuse.inputs['Color'].default_value = color
    fresnel.inputs['IOR'].default_value = 1.47

    mat.node_tree.links.new(light_path.outputs['Is Shadow Ray'], minimum.inputs[0])
    mat.node_tree.links.new(light_path.outputs['Is Reflection Ray'], minimum.inputs[1])
    mat.node_tree.links.new(minimum.outputs['Value'], mix_shader1.inputs['Fac'])

    mat.node_tree.links.new(glass.outputs['BSDF'], mix_shader1.inputs[1])
    mat.node_tree.links.new(translucent.outputs['BSDF'], mix_shader1.inputs[2])
    mat.node_tree.links.new(mix_shader1.outputs['Shader'], mix_shader2.inputs[1])

    mat.node_tree.links.new(fresnel.outputs['Fac'], mix_shader2.inputs['Fac'])
    mat.node_tree.links.new(diffuse.outputs['BSDF'], mix_shader2.inputs[2])

    mat.node_tree.links.new(mix_shader2.outputs['Shader'], output.inputs['Surface'])

def setup_background():
    bpy.ops.mesh.primitive_plane_add(location = (0, 0, 0), size = 100)
    plane = bpy.context.selected_objects[0]

    mat = bpy.data.materials.new('plane')
    mat.use_nodes = True
    mat.node_tree.nodes.clear()

    checker = mat.node_tree.nodes.new('ShaderNodeTexChecker')
    checker.inputs['Scale'].default_value = 40
    diffuse = mat.node_tree.nodes.new('ShaderNodeBsdfDiffuse')
    mat.node_tree.links.new(checker.outputs['Color'], diffuse.inputs['Color'])

    output = mat.node_tree.nodes.new('ShaderNodeOutputMaterial')
    mat.node_tree.links.new(diffuse.outputs['BSDF'], output.inputs['Surface'])

    plane.active_material = mat

def setup_camera():
    camera = bpy.data.objects['Camera']
    camera.location = (15, -12, 10)

clear_scene()
setup_denoising()
setup_background()
setup_camera()

red = bpy.data.materials.new('orb_red')
setup_orb_material(red, (1, 0, 0, 1))

green = bpy.data.materials.new('orb_green')
setup_orb_material(green, (0, 1, 0, 1))

blue = bpy.data.materials.new('orb_blue')
setup_orb_material(blue, (0, 0, 1, 1))

for i in range(100):
    radius = random.uniform(0.1, 1)
    bpy.ops.mesh.primitive_uv_sphere_add(radius = radius)
    sphere = bpy.context.selected_objects[0]

    x, y, z = random.uniform(-10, 20), random.uniform(-5, 5), random.uniform(0, 10)
    sphere.location = (x, y, z + radius)
    sphere.active_material = random.choice([red, green, blue])

    bpy.ops.object.mode_set(mode = 'EDIT')
    bpy.ops.mesh.faces_shade_smooth()
    bpy.ops.object.mode_set(mode = 'OBJECT')
