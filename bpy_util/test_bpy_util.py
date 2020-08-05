import math
from pytest import approx

from .bpy_util import camera_angle_to_origin

QUARTER_ROTATION = math.tau / 4
HALF_ROTATION = math.tau / 2
EIGHT_ROTATION = math.tau / 8
DIAGONAL_ABOVE_RAISE_ANGLE = 0.9553166
DIAGONAL_BELOW_RAISE_ANGLE = 2.1862760

def test_camera_angle_to_origin():
    assert camera_angle_to_origin((10, 0, 0)) == approx((QUARTER_ROTATION, 0, QUARTER_ROTATION))
    assert camera_angle_to_origin((-10, 0, 0)) == approx((QUARTER_ROTATION, 0, 3 * QUARTER_ROTATION))
    assert camera_angle_to_origin((0, 10, 0)) == approx((QUARTER_ROTATION, 0, HALF_ROTATION))
    assert camera_angle_to_origin((0, -10, 0)) == approx((QUARTER_ROTATION, 0, 0))
    assert camera_angle_to_origin((0, 0, 10)) == approx((0, 0, QUARTER_ROTATION))
    assert camera_angle_to_origin((0, 0, -10)) == approx((HALF_ROTATION, 0, QUARTER_ROTATION))

    assert camera_angle_to_origin((10, 10, 10)) == approx((DIAGONAL_ABOVE_RAISE_ANGLE, 0, 3 * EIGHT_ROTATION))
    assert camera_angle_to_origin((-10, 10, 10)) == approx((DIAGONAL_ABOVE_RAISE_ANGLE, 0, 5 * EIGHT_ROTATION))
    assert camera_angle_to_origin((10, -10, 10)) == approx((DIAGONAL_ABOVE_RAISE_ANGLE, 0, EIGHT_ROTATION))
    assert camera_angle_to_origin((10, 10, -10)) == approx((DIAGONAL_BELOW_RAISE_ANGLE, 0, 3 * EIGHT_ROTATION))
    assert camera_angle_to_origin((-10, -10, 10)) == approx((DIAGONAL_ABOVE_RAISE_ANGLE, 0, -EIGHT_ROTATION))
    assert camera_angle_to_origin((10, -10, -10)) == approx((DIAGONAL_BELOW_RAISE_ANGLE, 0, EIGHT_ROTATION))
    assert camera_angle_to_origin((-10, 10, -10)) == approx((DIAGONAL_BELOW_RAISE_ANGLE, 0, 5 * EIGHT_ROTATION))
    assert camera_angle_to_origin((-10, -10, -10)) == approx((DIAGONAL_BELOW_RAISE_ANGLE, 0, -EIGHT_ROTATION))
