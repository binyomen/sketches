import bpy
import functools
import math
import os
import random
import sys

sys.path.append(os.path.realpath('..'))
from bpy_util import camera_angle_to_origin, enable_denoising

def clear_scene():
    bpy.data.objects.remove(bpy.data.objects['Cube'])
    bpy.data.objects.remove(bpy.data.objects['Light'])

# Technique adapted from https://www.youtube.com/watch?v=5OUpqvx6RE8.
def setup_orb_material(mat, color):
    mat.use_nodes = True
    mat.node_tree.nodes.clear()

    surface = mat.node_tree.nodes.new('ShaderNodeBsdfPrincipled')
    volume = mat.node_tree.nodes.new('ShaderNodeVolumePrincipled')
    output = mat.node_tree.nodes.new('ShaderNodeOutputMaterial')

    surface.inputs['Roughness'].default_value = 0
    surface.inputs['Transmission'].default_value = 1
    surface.inputs['IOR'].default_value = 1.45

    volume.inputs['Color'].default_value = color
    volume.inputs['Density'].default_value = 3

    mat.node_tree.links.new(surface.outputs['BSDF'], output.inputs['Surface'])
    mat.node_tree.links.new(volume.outputs['Volume'], output.inputs['Volume'])

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
    camera.location = (16, 21, 15)
    camera.rotation_euler = camera_angle_to_origin(camera.location)

# Cache the result so all lights have references to the same underlying data.
@functools.lru_cache(maxsize = None)
def create_light_data():
    light_data = bpy.data.lights.new(name = 'orb_light_data', type = 'POINT')
    light_data.energy = 100
    light_data.shadow_soft_size = 0.001
    return light_data

def create_light(location):
    light_data = create_light_data()
    light = bpy.data.objects.new(name = 'orb_light', object_data = light_data)
    light.location = location

    bpy.context.collection.objects.link(light)

clear_scene()
enable_denoising()
setup_background()
setup_camera()

red = bpy.data.materials.new('orb_red')
setup_orb_material(red, (1, 0.25, 0.25, 1))

green = bpy.data.materials.new('orb_green')
setup_orb_material(green, (0.25, 1, 0.25, 1))

blue = bpy.data.materials.new('orb_blue')
setup_orb_material(blue, (0.25, 0.25, 1, 1))

for i in range(100):
    radius = random.uniform(0.1, 1)
    bpy.ops.mesh.primitive_uv_sphere_add(radius = radius)
    sphere = bpy.context.selected_objects[0]

    x, y, z = random.uniform(-5, 10), random.uniform(-5, 10), random.uniform(0, 10)
    sphere.location = (x, y, z + radius)
    sphere.active_material = random.choice([red, green, blue])

    bpy.ops.object.mode_set(mode = 'EDIT')
    bpy.ops.mesh.faces_shade_smooth()
    bpy.ops.object.mode_set(mode = 'OBJECT')

    create_light(sphere.location)

bpy.context.scene.cycles.samples = 512

blend_file_path = os.path.join(os.path.dirname(os.path.realpath(__file__)), 'orbs.blend')
bpy.ops.wm.save_as_mainfile(filepath = blend_file_path)
