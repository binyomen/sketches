import bpy
import random

# Get rid of the initial cube.
bpy.data.objects.remove(bpy.data.objects['Cube'])

def setup_material(mat, color):
    mat.use_nodes = True

    bsdf = mat.node_tree.nodes.new('ShaderNodeBsdfPrincipled')

    bsdf.inputs['Base Color'].default_value = color
    bsdf.inputs['Subsurface'].default_value = 0.5
    bsdf.inputs['Subsurface Color'].default_value = color
    bsdf.inputs['Metallic'].default_value = 0.6
    bsdf.inputs['Sheen'].default_value = 0.5
    bsdf.inputs['Alpha'].default_value = 0.6

    output = mat.node_tree.nodes.get('Material Output')
    mat.node_tree.links.new(output.inputs[0], bsdf.outputs[0])

red = bpy.data.materials.new('orb_red')
setup_material(red, (1, 0, 0, 1))

green = bpy.data.materials.new('orb_green')
setup_material(green, (0, 1, 0, 1))

blue = bpy.data.materials.new('orb_blue')
setup_material(blue, (0, 0, 1, 1))

for i in range(20):
    bpy.ops.mesh.primitive_uv_sphere_add()
    sphere = bpy.context.selected_objects[0]

    x, y, z = random.uniform(-5, 5), random.uniform(-5, 5), random.uniform(-5, 5)
    sphere.location = (x, y, z)

    material = random.choice([red, green, blue])
    sphere.data.materials.clear()
    sphere.active_material = material

    bpy.ops.object.mode_set(mode = 'EDIT')
    bpy.ops.mesh.faces_shade_smooth()
    bpy.ops.object.mode_set(mode = 'OBJECT')
