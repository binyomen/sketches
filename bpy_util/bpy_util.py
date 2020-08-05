import contextlib
import math

with contextlib.suppress(Exception):
    import bpy

def camera_angle_to_origin(location):
    """
    Get the Euler rotation angle for the camera to point towards the origin
    from ``location``.
    """

    x, y, z = location

    # Add tau/4 since atan2 will give us the angle from the origin to the
    # location, which means we need to add tau/2, and also the camera starts at
    # tau/4, so subtract that and arrive at +tau/4.
    angle_z = math.atan2(y, x) + (math.tau / 4)
    angle_x = math.atan2(math.sqrt(x**2 + y**2), z)

    # z angle should be applied first by Blender.
    return (angle_x, 0, angle_z)

def enable_denoising():
    """
    Add a basic set of compositor nodes to denoise the render in CYCLES.
    """

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
