import Mathlib.Data.ZMod.Basic

set_option linter.dupNamespace false
set_option linter.unnecessarySeqFocus false

namespace UOR.Algebra.O256

abbrev Byte := ZMod 256

structure Oct256 where
  a0 : Byte
  a1 : Byte
  a2 : Byte
  a3 : Byte
  a4 : Byte
  a5 : Byte
  a6 : Byte
  a7 : Byte
  deriving DecidableEq, Repr

@[ext] theorem oct256_ext : ∀ {x y : Oct256},
    x.a0 = y.a0 → x.a1 = y.a1 → x.a2 = y.a2 → x.a3 = y.a3 →
    x.a4 = y.a4 → x.a5 = y.a5 → x.a6 = y.a6 → x.a7 = y.a7 → x = y := by
  intro x y h0 h1 h2 h3 h4 h5 h6 h7
  cases x <;> cases y <;> simp_all

abbrev O256 := Oct256

def zero : O256 := ⟨0, 0, 0, 0, 0, 0, 0, 0⟩
def one : O256 := ⟨1, 0, 0, 0, 0, 0, 0, 0⟩

def add : O256 → O256 → O256
| ⟨a0, a1, a2, a3, a4, a5, a6, a7⟩, ⟨b0, b1, b2, b3, b4, b5, b6, b7⟩ =>
    ⟨a0 + b0, a1 + b1, a2 + b2, a3 + b3, a4 + b4, a5 + b5, a6 + b6, a7 + b7⟩

def neg : O256 → O256
| ⟨a0, a1, a2, a3, a4, a5, a6, a7⟩ =>
    ⟨-a0, -a1, -a2, -a3, -a4, -a5, -a6, -a7⟩

def sub : O256 → O256 → O256
| x, y => add x (neg y)

def mul (x y : O256) : O256 :=
  match x, y with
  | ⟨a0, a1, a2, a3, a4, a5, a6, a7⟩, ⟨b0, b1, b2, b3, b4, b5, b6, b7⟩ =>
      ⟨
        a0 * b0 - a1 * b1 - a2 * b2 - a3 * b3 - a4 * b4 - a5 * b5 - a6 * b6 - a7 * b7,
        a0 * b1 + a1 * b0 + a2 * b3 - a3 * b2 + a4 * b5 - a5 * b4 - a6 * b7 + a7 * b6,
        a0 * b2 - a1 * b3 + a2 * b0 + a3 * b1 + a4 * b6 + a5 * b7 - a6 * b4 - a7 * b5,
        a0 * b3 + a1 * b2 - a2 * b1 + a3 * b0 + a4 * b7 - a5 * b6 + a6 * b5 - a7 * b4,
        a0 * b4 - a1 * b5 - a2 * b6 - a3 * b7 + a4 * b0 + a5 * b1 + a6 * b2 + a7 * b3,
        a0 * b5 + a1 * b4 - a2 * b7 + a3 * b6 - a4 * b1 + a5 * b0 - a6 * b3 + a7 * b2,
        a0 * b6 + a1 * b7 + a2 * b4 - a3 * b5 - a4 * b2 + a5 * b3 + a6 * b0 - a7 * b1,
        a0 * b7 - a1 * b6 + a2 * b5 + a3 * b4 - a4 * b3 - a5 * b2 + a6 * b1 + a7 * b0
      ⟩

instance : Zero O256 := ⟨zero⟩
instance : One O256 := ⟨one⟩
instance : Add O256 := ⟨add⟩
instance : Neg O256 := ⟨neg⟩
instance : Sub O256 := ⟨sub⟩
instance : Mul O256 := ⟨mul⟩

def conj : O256 → O256
| ⟨a0, a1, a2, a3, a4, a5, a6, a7⟩ =>
    ⟨a0, -a1, -a2, -a3, -a4, -a5, -a6, -a7⟩

def norm : O256 → Byte
| ⟨a0, a1, a2, a3, a4, a5, a6, a7⟩ =>
    a0 * a0 + a1 * a1 + a2 * a2 + a3 * a3 + a4 * a4 + a5 * a5 + a6 * a6 + a7 * a7

def scalar : Byte → O256
| a => ⟨a, 0, 0, 0, 0, 0, 0, 0⟩

theorem mul_conj_is_scalar (x : O256) : x * conj x = scalar (norm x) := by
  cases x with
  | mk a0 a1 a2 a3 a4 a5 a6 a7 =>
      decide

def e0 : O256 := one
def e1 : O256 := ⟨0, 1, 0, 0, 0, 0, 0, 0⟩
def e2 : O256 := ⟨0, 0, 1, 0, 0, 0, 0, 0⟩
def e3 : O256 := ⟨0, 0, 0, 1, 0, 0, 0, 0⟩
def e4 : O256 := ⟨0, 0, 0, 0, 1, 0, 0, 0⟩
def e5 : O256 := ⟨0, 0, 0, 0, 0, 1, 0, 0⟩
def e6 : O256 := ⟨0, 0, 0, 0, 0, 0, 1, 0⟩
def e7 : O256 := ⟨0, 0, 0, 0, 0, 0, 0, 1⟩

@[simp] theorem conj_e0 : conj e0 = e0 := by rfl
@[simp] theorem conj_e1 : conj e1 = -e1 := by rfl
@[simp] theorem conj_e2 : conj e2 = -e2 := by rfl
@[simp] theorem conj_e3 : conj e3 = -e3 := by rfl
@[simp] theorem conj_e4 : conj e4 = -e4 := by rfl
@[simp] theorem conj_e5 : conj e5 = -e5 := by rfl
@[simp] theorem conj_e6 : conj e6 = -e6 := by rfl
@[simp] theorem conj_e7 : conj e7 = -e7 := by rfl

@[simp] theorem conj_conj (x : O256) : conj (conj x) = x := by
  cases x <;> simp [conj]

@[simp] theorem mul_e1_e1 : e1 * e1 = -e0 := by rfl
@[simp] theorem mul_e2_e2 : e2 * e2 = -e0 := by rfl
@[simp] theorem mul_e3_e3 : e3 * e3 = -e0 := by rfl
@[simp] theorem mul_e4_e4 : e4 * e4 = -e0 := by rfl
@[simp] theorem mul_e5_e5 : e5 * e5 = -e0 := by rfl
@[simp] theorem mul_e6_e6 : e6 * e6 = -e0 := by rfl
@[simp] theorem mul_e7_e7 : e7 * e7 = -e0 := by rfl

@[simp] theorem mul_e1_e2 : e1 * e2 = e3 := by rfl
@[simp] theorem mul_e2_e3 : e2 * e3 = e1 := by rfl
@[simp] theorem mul_e3_e1 : e3 * e1 = e2 := by rfl

@[simp] theorem mul_e2_e1 : e2 * e1 = -e3 := by rfl
@[simp] theorem mul_e3_e2 : e3 * e2 = -e1 := by rfl
@[simp] theorem mul_e1_e3 : e1 * e3 = -e2 := by rfl

example : (e1 * e2) * e4 = e7 := by decide
example : e1 * (e2 * e4) = -e7 := by decide

theorem not_assoc_example : (e1 * e2) * e4 ≠ e1 * (e2 * e4) := by
  decide

end UOR.Algebra.O256
