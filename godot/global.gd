extends Node

## Holds global data and various static helper functions

@export var DataTypes: Dictionary = {
	0: {
		"type": "trigger",
		"desc": "Used as a general purpose trigger to activate another node."
	},
	1: {
		"type": "number",
		"desc": "Represents a numerical value. Always floating point"
	},
	2: {
		"type": "text",
		"desc": "Represents arbitrary text data."
	}
}

static func foo() -> void:
	print_debug("Hi")
