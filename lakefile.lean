import Lake
open Lake DSL

package uor where
  leanOptions := #[
    ⟨`autoImplicit, false⟩
  ]
  preferReleaseBuild := true

require mathlib from git
  "https://github.com/leanprover-community/mathlib4.git"@"v4.16.0"

@[default_target]
lean_lib UOR where
  srcDir := "lean4"
