import bpy
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

def setup_material(mat, color):
    mat.use_nodes = True
    mat.node_tree.nodes.clear()

    bsdf = mat.node_tree.nodes.new('ShaderNodeBsdfPrincipled')

    bsdf.inputs['Base Color'].default_value = color
    bsdf.inputs['Subsurface'].default_value = 0.5
    bsdf.inputs['Subsurface Color'].default_value = color
    bsdf.inputs['Metallic'].default_value = 0.6
    bsdf.inputs['Sheen'].default_value = 0.5
    bsdf.inputs['Alpha'].default_value = 0.6

    output = mat.node_tree.nodes.new('ShaderNodeOutputMaterial')
    mat.node_tree.links.new(bsdf.outputs['BSDF'], output.inputs['Surface'])

clear_scene()
setup_denoising()

red = bpy.data.materials.new('orb_red')
setup_material(red, (1, 0, 0, 1))

green = bpy.data.materials.new('orb_green')
setup_material(green, (0, 1, 0, 1))

blue = bpy.data.materials.new('orb_blue')
setup_material(blue, (0, 0, 1, 1))

for i in range(20):
    bpy.ops.mesh.primitive_uv_sphere_add(radius = random.uniform(0.1, 1))
    sphere = bpy.context.selected_objects[0]

    x, y, z = random.uniform(-5, 5), random.uniform(-5, 5), random.uniform(-5, 5)
    sphere.location = (x, y, z)

    material = random.choice([red, green, blue])
    sphere.data.materials.clear()
    sphere.active_material = material

    bpy.ops.object.mode_set(mode = 'EDIT')
    bpy.ops.mesh.faces_shade_smooth()
    bpy.ops.object.mode_set(mode = 'OBJECT')
