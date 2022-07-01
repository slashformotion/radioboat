package utils

import (
	"errors"

	"github.com/manifoldco/promptui"
)

func GetInteractiveBooleanPrompt(label string) (*promptui.Prompt, error) {
	validate := func(input string) error {
		if input != "y" && input != "n" {
			return errors.New("invalid input")
		}
		return nil
	}
	return &promptui.Prompt{
		Label:    label + " (answer with \"y\" or \"n\")",
		Validate: validate,
	}, nil
}
