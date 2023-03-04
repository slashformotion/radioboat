package tui

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import "github.com/charmbracelet/lipgloss"

// Stylesheets
var (
	header_center_s = lipgloss.NewStyle().
			Foreground(lipgloss.Color("233")).
			Background(lipgloss.Color("147")).
			Align(lipgloss.Center)

	header_status_s = lipgloss.NewStyle().Inherit(header_center_s).
			Foreground(lipgloss.Color("233")).
			Background(lipgloss.Color("#FF5F87")).
			PaddingLeft(1).
			PaddingRight(1)

	header_volume_s = lipgloss.NewStyle().Inherit(header_center_s).
			Foreground(lipgloss.Color("233")).
			Background(lipgloss.Color("#A550DF")).
			PaddingLeft(1).
			PaddingRight(1)

	list_selected_s = lipgloss.NewStyle().Bold(true)

	docStyle = lipgloss.NewStyle().Padding(1, 2, 1, 2)
)
