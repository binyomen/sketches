[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [String] $SketchName,
    [UInt32] $Frame = 1,
    [String] $FileName = 'frame',

    [ValidateSet('BLENDER_EEVEE', 'BLENDER_WORKBENCH', 'CYCLES')]
    [String] $Engine = 'BLENDER_EEVEE'
)

$sketchPath = "$PSScriptRoot\$SketchName"
blender --background --engine $Engine --render-output "$sketchPath\$FileName" --python "$sketchPath\run.py" --render-frame $Frame
