param(
    [Parameter(Mandatory=$True)]
    [String] $SketchName
)
$SketchPath = Resolve-Path $SketchName

processing-java --sketch=$SketchPath --run
