use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum BooleanNodes {
    #[mutagen(gen_weight = branch_node_weight)]
    UNFloatLess {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    UNFloatMore {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    UNFloatBetween {
        child_value: Box<UNFloatNodes>,
        child_range_a: Box<UNFloatNodes>,
        child_range_b: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    SNFloatLess {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    SNFloatMore {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    SNFloatBetween {
        child_value: Box<SNFloatNodes>,
        child_range_a: Box<SNFloatNodes>,
        child_range_b: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    SNFloatSign { child: Box<SNFloatNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    And {
        child_a: Box<BooleanNodes>,
        child_b: Box<BooleanNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Or {
        child_a: Box<BooleanNodes>,
        child_b: Box<BooleanNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    Not { child: Box<BooleanNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    BitColorHas {
        child_a: Box<BitColorNodes>,
        child_b: Box<BitColorNodes>,
    },
    // #[mutagen(mut_reroll = 0.9)]
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: Boolean },
    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<BooleanNodes>,
        child_state: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<BooleanNodes>,
        child_b: Box<BooleanNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    ByteEquals {
        child_a: Box<ByteNodes>,
        child_b: Box<ByteNodes>,
    },
    // #[mutagen(gen_weight = branch_node_weight)]
    // Majority {
    //     child: Box<BooleanNodes>,
    //     point_set: PointSet,
    // },
}

impl Node for BooleanNodes {
    type Output = Boolean;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use BooleanNodes::*;

        match self {
            UNFloatLess { child_a, child_b } => Boolean {
                value: child_a.compute(compute_arg.reborrow()).into_inner()
                    < child_b.compute(compute_arg.reborrow()).into_inner(),
            },
            UNFloatMore { child_a, child_b } => Boolean {
                value: child_a.compute(compute_arg.reborrow()).into_inner()
                    > child_b.compute(compute_arg.reborrow()).into_inner(),
            },
            UNFloatBetween {
                child_value,
                child_range_a,
                child_range_b,
            } => {
                let range_a = child_range_a.compute(compute_arg.reborrow()).into_inner();
                let range_b = child_range_b.compute(compute_arg.reborrow()).into_inner();

                let max = range_a.max(range_b);
                let min = range_a.min(range_b);

                let value = child_value.compute(compute_arg.reborrow()).into_inner();

                Boolean {
                    value: value > min && value < max,
                }
            }
            SNFloatLess { child_a, child_b } => Boolean {
                value: child_a.compute(compute_arg.reborrow()).into_inner()
                    < child_b.compute(compute_arg.reborrow()).into_inner(),
            },
            SNFloatMore { child_a, child_b } => Boolean {
                value: child_a.compute(compute_arg.reborrow()).into_inner()
                    > child_b.compute(compute_arg.reborrow()).into_inner(),
            },
            SNFloatBetween {
                child_value,
                child_range_a,
                child_range_b,
            } => {
                let range_a = child_range_a.compute(compute_arg.reborrow()).into_inner();
                let range_b = child_range_b.compute(compute_arg.reborrow()).into_inner();

                let max = range_a.max(range_b);
                let min = range_a.min(range_b);

                let value = child_value.compute(compute_arg.reborrow()).into_inner();

                Boolean {
                    value: value > min && value < max,
                }
            }
            SNFloatSign { child } => Boolean {
                value: child.compute(compute_arg.reborrow()).into_inner() >= 0.0,
            },
            And { child_a, child_b } => Boolean {
                value: child_a.compute(compute_arg.reborrow()).into_inner()
                    && child_b.compute(compute_arg.reborrow()).into_inner(),
            },
            Or { child_a, child_b } => Boolean {
                value: child_a.compute(compute_arg.reborrow()).into_inner()
                    || child_b.compute(compute_arg.reborrow()).into_inner(),
            },
            Not { child } => Boolean {
                value: !child.compute(compute_arg.reborrow()).into_inner(),
            },
            BitColorHas { child_a, child_b } => Boolean {
                value: child_a
                    .compute(compute_arg.reborrow())
                    .has_color(child_b.compute(compute_arg.reborrow())),
            },
            Constant { value } => *value,
            // Random => Boolean::generate(state),
            ModifyState { child, child_state } => child.compute(ComArg {
                coordinate_set: child_state.compute(compute_arg.reborrow()),
                ..compute_arg.reborrow()
            }),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(compute_arg.reborrow()).into_inner() {
                    child_a.compute(compute_arg.reborrow())
                } else {
                    child_b.compute(compute_arg.reborrow())
                }
            }
            ByteEquals { child_a, child_b } => Boolean {
                value: child_a.compute(compute_arg.reborrow()).into_inner()
                    == child_b.compute(compute_arg.reborrow()).into_inner(),
            },
            // Majority {
            //     child,
            //     point_set,
            // } => {
            //     let mut true_count = 0;
            //     let offsets = point_set.get_offsets(CONSTS.cell_array_width, CONSTS.cell_array_height);

            //     //this might blow up
            //     for point in &offsets {
            //         let offset_state = UpdateState {
            //             coordinate_set: state.coordinate_set.get_coord_shifted(
            //                 point.x(),
            //                 point.y(),
            //                 SNFloat::new(0.0),
            //             ),
            //             history: state.history,
            //         };

            //         if child.compute(offset_state).into_inner() {
            //             true_count += 1;
            //         }
            //     }

            //     Boolean {
            //         value: true_count > offsets.len() / 2,
            //     }
            // }
        }
    }
}

impl<'a> Updatable<'a> for BooleanNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum NibbleNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant {
        value: Nibble,
    },
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = branch_node_weight)]
    Add {
        child_a: Box<NibbleNodes>,
        child_b: Box<NibbleNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<NibbleNodes>,
        child_b: Box<NibbleNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Divide {
        child_value: Box<NibbleNodes>,
        child_divisor: Box<NibbleNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Modulus {
        child_value: Box<NibbleNodes>,
        child_divisor: Box<NibbleNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    FromBooleans {
        a: Box<BooleanNodes>,
        b: Box<BooleanNodes>,
        c: Box<BooleanNodes>,
        d: Box<BooleanNodes>,
    },
    FromByteModulo {
        child: Box<ByteNodes>,
    },
    FromByteDivide {
        child: Box<ByteNodes>,
    },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<NibbleNodes>,
        child_b: Box<NibbleNodes>,
    },
    // InvertNormalised { child: Box<NibbleNodes> },
}

impl Node for NibbleNodes {
    type Output = Nibble;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use NibbleNodes::*;

        match self {
            Constant { value } => *value,
            // Random => Nibble::generate(state),
            Add { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .circular_add(child_b.compute(compute_arg.reborrow())),
            Multiply { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .circular_multiply(child_b.compute(compute_arg.reborrow())),
            Divide {
                child_value,
                child_divisor,
            } => child_value
                .compute(compute_arg.reborrow())
                .divide(child_divisor.compute(compute_arg.reborrow())),
            Modulus {
                child_value,
                child_divisor,
            } => child_value
                .compute(compute_arg.reborrow())
                .modulus(child_divisor.compute(compute_arg.reborrow())),
            FromBooleans { a, b, c, d } => {
                let mut value = 0;

                if a.compute(compute_arg.reborrow()).into_inner() {
                    value += 1;
                }
                if b.compute(compute_arg.reborrow()).into_inner() {
                    value += 2;
                }
                if c.compute(compute_arg.reborrow()).into_inner() {
                    value += 4;
                }
                if d.compute(compute_arg.reborrow()).into_inner() {
                    value += 8;
                }

                Nibble::new(value)
            }
            FromByteModulo { child } => {
                Nibble::new_circular(child.compute(compute_arg.reborrow()).into_inner())
            }
            FromByteDivide { child } => {
                Nibble::new(child.compute(compute_arg.reborrow()).into_inner() / Nibble::MODULUS)
            }
            FromGametic => {
                Nibble::new_circular(compute_arg.coordinate_set.get_byte_t().into_inner())
            }
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(compute_arg.reborrow()).into_inner() {
                    child_a.compute(compute_arg.reborrow())
                } else {
                    child_b.compute(compute_arg.reborrow())
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for NibbleNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum ByteNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: Byte },
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = branch_node_weight)]
    Add {
        child_a: Box<ByteNodes>,
        child_b: Box<ByteNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<ByteNodes>,
        child_b: Box<ByteNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    MultiplyNibbles {
        child_a: Box<NibbleNodes>,
        child_b: Box<NibbleNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Divide {
        child_value: Box<ByteNodes>,
        child_divisor: Box<ByteNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Modulus {
        child_value: Box<ByteNodes>,
        child_divisor: Box<ByteNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromIterativeResult { child: Box<IterativeFunctionNodes> },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<ByteNodes>,
        child_b: Box<ByteNodes>,
    },
}

impl Node for ByteNodes {
    type Output = Byte;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use ByteNodes::*;

        match self {
            Constant { value } => *value,
            // Random => Byte::generate(state),
            Add { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .circular_add(child_b.compute(compute_arg.reborrow())),
            Multiply { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .circular_multiply(child_b.compute(compute_arg.reborrow())),
            MultiplyNibbles { child_a, child_b } => Byte::new(
                child_a.compute(compute_arg.reborrow()).into_inner()
                    * child_b.compute(compute_arg.reborrow()).into_inner(),
            ),
            Divide {
                child_value,
                child_divisor,
            } => child_value
                .compute(compute_arg.reborrow())
                .divide(child_divisor.compute(compute_arg.reborrow())),
            Modulus {
                child_value,
                child_divisor,
            } => child_value
                .compute(compute_arg.reborrow())
                .modulus(child_divisor.compute(compute_arg.reborrow())),
            FromIterativeResult { child } => child.compute(compute_arg).iter_final,
            FromGametic => compute_arg.coordinate_set.get_byte_t(),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(compute_arg.reborrow()).into_inner() {
                    child_a.compute(compute_arg.reborrow())
                } else {
                    child_b.compute(compute_arg.reborrow())
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for ByteNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum UIntNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: UInt },
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = branch_node_weight)]
    Add {
        child_a: Box<UIntNodes>,
        child_b: Box<UIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<UIntNodes>,
        child_b: Box<UIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Divide {
        child_value: Box<UIntNodes>,
        child_divisor: Box<UIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Modulus {
        child_value: Box<UIntNodes>,
        child_divisor: Box<UIntNodes>,
    },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<UIntNodes>,
        child_b: Box<UIntNodes>,
    },
}

impl Node for UIntNodes {
    type Output = UInt;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use UIntNodes::*;

        match self {
            Constant { value } => *value,
            // Random => UInt::generate(state),
            Add { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .circular_add(child_b.compute(compute_arg.reborrow())),
            Multiply { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .circular_multiply(child_b.compute(compute_arg.reborrow())),
            Divide {
                child_value,
                child_divisor,
            } => child_value
                .compute(compute_arg.reborrow())
                .divide(child_divisor.compute(compute_arg.reborrow())),
            Modulus {
                child_value,
                child_divisor,
            } => child_value
                .compute(compute_arg.reborrow())
                .modulus(child_divisor.compute(compute_arg.reborrow())),
            FromGametic => UInt::new(compute_arg.coordinate_set.t as u32),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(compute_arg.reborrow()).into_inner() {
                    child_a.compute(compute_arg.reborrow())
                } else {
                    child_b.compute(compute_arg.reborrow())
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for UIntNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum SIntNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: SInt },
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = branch_node_weight)]
    Add {
        child_a: Box<SIntNodes>,
        child_b: Box<SIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<SIntNodes>,
        child_b: Box<SIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Divide {
        child_value: Box<SIntNodes>,
        child_divisor: Box<SIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Modulus {
        child_value: Box<SIntNodes>,
        child_divisor: Box<SIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<SIntNodes>,
        child_b: Box<SIntNodes>,
    },
}

impl Node for SIntNodes {
    type Output = SInt;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use SIntNodes::*;

        match self {
            Constant { value } => *value,
            // Random => SInt::generate(state),
            Add { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .circular_add(child_b.compute(compute_arg.reborrow())),
            Multiply { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .circular_multiply(child_b.compute(compute_arg.reborrow())),
            Divide {
                child_value,
                child_divisor,
            } => child_value
                .compute(compute_arg.reborrow())
                .divide(child_divisor.compute(compute_arg.reborrow())),
            Modulus {
                child_value,
                child_divisor,
            } => child_value
                .compute(compute_arg.reborrow())
                .modulus(child_divisor.compute(compute_arg.reborrow())),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(compute_arg.reborrow()).into_inner() {
                    child_a.compute(compute_arg.reborrow())
                } else {
                    child_b.compute(compute_arg.reborrow())
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for SIntNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}
