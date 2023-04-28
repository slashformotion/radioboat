package utils_test

import (
	"testing"

	"github.com/slashformotion/radioboat/utils"
)

func Test(t *testing.T) {
	testCases := []struct {
		desc             string
		v, min, max, out int
	}{
		{
			desc: "",
			v:    10, min: 5, max: 15, out: 10,
		}, {
			desc: "",
			v:    4, min: 5, max: 15, out: 5,
		},
		{
			desc: "",
			v:    18, min: 5, max: 15, out: 15,
		},
	}
	for _, tC := range testCases {
		t.Run(tC.desc, func(t *testing.T) {
			if res := utils.ClampInts(tC.v, tC.min, tC.max); tC.out != res {
				t.Errorf("expected %v , got %v", tC.out, res)
			}
		})
	}
}
