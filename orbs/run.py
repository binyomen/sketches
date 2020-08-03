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

def setup_orb_material(mat, color):
    mat.use_nodes = True
    mat.node_tree.nodes.clear()

    bsdf = mat.node_tree.nodes.new('ShaderNodeBsdfGlass')

    bsdf.inputs['Color'].default_value = color
    bsdf.inputs['Roughness'].default_value = 0.1
    bsdf.inputs['IOR'].default_value = 1.7

    output = mat.node_tree.nodes.new('ShaderNodeOutputMaterial')
    mat.node_tree.links.new(bsdf.outputs['BSDF'], output.inputs['Surface'])

def setup_background():
    bpy.ops.mesh.primitive_plane_add(location = (0, 0, 0), size = 100)
    plane = bpy.context.selected_objects[0]

    mat = bpy.data.materials.new('plane')
    mat.use_nodes = True
    mat.node_tree.nodes.clear()

    checker = mat.node_tree.nodes.new('ShaderNodeTexChecker')
    checker.inputs['Scale'].default_value = 40
    bsdf = mat.node_tree.nodes.new('ShaderNodeBsdfDiffuse')
    mat.node_tree.links.new(checker.outputs['Color'], bsdf.inputs['Color'])

    output = mat.node_tree.nodes.new('ShaderNodeOutputMaterial')
    mat.node_tree.links.new(bsdf.outputs['BSDF'], output.inputs['Surface'])

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
