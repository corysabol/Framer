extends GraphNode

## Handles connecting and pairing with the Frame glasses over BLE

func pair(device_name: String) -> bool:
	# TODO: hook into and call the rust godot-frame function for pairing
	return false

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
