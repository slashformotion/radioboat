package utils

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
