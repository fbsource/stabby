//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use self::err_size_0::DiscriminantProviderWithUnit;

pub use super::*;

pub struct DiscriminantProvider<
    UnionSize: Unsigned,
    Budget,
    OkFv: IForbiddenValues,
    OkUb: IBitMask,
    ErrFv: IForbiddenValues,
    ErrUb: IBitMask,
    ErrSize: Unsigned,
    ErrAlign: PowerOf2,
    ErrOffset: Unsigned,
>(
    core::marker::PhantomData<(
        UnionSize,
        Budget,
        OkFv,
        OkUb,
        ErrFv,
        ErrUb,
        ErrSize,
        ErrAlign,
        ErrOffset,
    )>,
);
pub struct DiscriminantProviderBranch<
    Provider,
    ErrFvInOkUb: ISingleForbiddenValue,
    OkFvInErrUb: ISingleForbiddenValue,
    UbIntersect: IBitMask,
>(core::marker::PhantomData<(Provider, ErrFvInOkUb, OkFvInErrUb, UbIntersect)>);
/// Prevents the compiler from doing infinite recursion when evaluating `IDiscriminantProvider`
type DefaultRecursionBudget = T<T<T<T<T<T<T<T<H>>>>>>>>;
// ENTER LOOP ON Budget
impl<Ok: IStable, Err: IStable, EI: Unsigned, EB: Bit> IDiscriminantProviderInner
    for (Ok, Err, UInt<EI, EB>)
where
    DiscriminantProvider<
        UnionSize<Ok, Err, U0, U0>,
        DefaultRecursionBudget,
        Ok::ForbiddenValues,
        UnionMemberUnusedBits<Ok, Err, U0>,
        Err::ForbiddenValues,
        Err::UnusedBits,
        Err::Size,
        Err::Align,
        U0,
    >: IDiscriminantProviderInner,
{
    same_as!(
        DiscriminantProvider<
            UnionSize<Ok, Err, U0, U0>,
            DefaultRecursionBudget,
            Ok::ForbiddenValues,
            UnionMemberUnusedBits<Ok, Err, U0>,
            Err::ForbiddenValues,
            Err::UnusedBits,
            Err::Size,
            Err::Align,
            U0,
        >
    );
}
// EXIT LOOP
impl<
        UnionSize: Unsigned,
        OkFv: IForbiddenValues,
        OkUb: IBitMask,
        ErrFv: IForbiddenValues,
        ErrUb: IBitMask,
        ErrSize: Unsigned,
        ErrAlign: PowerOf2,
        ErrOffset: Unsigned,
    > IDiscriminantProviderInner
    for DiscriminantProvider<UnionSize, H, OkFv, OkUb, ErrFv, ErrUb, ErrSize, ErrAlign, ErrOffset>
{
    same_as!(DiscriminantProviderWithUnit<End, End>);
}

type UnusedBits<UnionSize, ErrUb, ErrSize, ErrOffset> = <<<ErrOffset as Unsigned>::Padding as IStable>::UnusedBits as IBitMask>::BitOr<<<ErrUb as IBitMask>::Shift<ErrOffset> as IBitMask>::BitOr<<<tyeval!(UnionSize - (ErrSize + ErrOffset)) as Unsigned>::Padding as IStable>::UnusedBits>>;

/// Branch on whether some forbidden values for Err fit inside Ok's unused bits
impl<
UnionSize: Unsigned,
Budget,
OkFv: IForbiddenValues,
OkUb: IBitMask,
ErrFv: IForbiddenValues,
ErrUb: IBitMask,
ErrSize: Unsigned,
ErrAlign: PowerOf2,
ErrOffset: Unsigned,
>
    IDiscriminantProviderInner for DiscriminantProvider<UnionSize, T<Budget>, OkFv, OkUb, ErrFv, ErrUb, ErrSize, ErrAlign, ErrOffset>
where
DiscriminantProviderBranch<
        Self,
        <<ErrFv::Shift<ErrOffset> as IForbiddenValues>::SelectFrom<
            OkUb
        > as ISingleForbiddenValue>::Resolve,
        <OkFv::SelectFrom<
            UnusedBits<UnionSize, ErrUb, ErrSize, ErrOffset>
        > as ISingleForbiddenValue>::Resolve,
        OkUb::BitAnd<UnusedBits<UnionSize, ErrUb, ErrSize, ErrOffset>>
 >: IDiscriminantProviderInner,
{
    same_as!(DiscriminantProviderBranch<
        Self,
        <<ErrFv::Shift<ErrOffset> as IForbiddenValues>::SelectFrom<
            OkUb
        > as ISingleForbiddenValue>::Resolve,
        <OkFv::SelectFrom<
            UnusedBits<UnionSize, ErrUb, ErrSize, ErrOffset>
        > as ISingleForbiddenValue>::Resolve,
        OkUb::BitAnd<UnusedBits<UnionSize, ErrUb, ErrSize, ErrOffset>>
 >);
}

/// If some forbidden values for Err fit inside Ok's unused bits, exit the recursion
impl<
        UnionSize: Unsigned,
        Budget,
        OkFv: IForbiddenValues,
        OkUb: IBitMask,
        ErrFv: IForbiddenValues,
        ErrUb: IBitMask,
        ErrSize: Unsigned,
        ErrAlign: PowerOf2,
        ErrOffset: Unsigned,
        Offset: Unsigned,
        V: Unsigned,
        Tail: IForbiddenValues + ISingleForbiddenValue + IntoValueIsErr,
        OkFvInErrUb: ISingleForbiddenValue,
        UbIntersect: IBitMask,
    > IDiscriminantProviderInner
    for DiscriminantProviderBranch<
        DiscriminantProvider<
            UnionSize,
            Budget,
            OkFv,
            OkUb,
            ErrFv,
            ErrUb,
            ErrSize,
            ErrAlign,
            ErrOffset,
        >,
        Array<Offset, V, Tail>,
        OkFvInErrUb,
        UbIntersect,
    >
{
    type ErrShift = ErrOffset;
    type Discriminant = Not<<Array<Offset, V, Tail> as IntoValueIsErr>::ValueIsErr>;
    type NicheExporter = NicheExporter<End, UbIntersect, Saturator>;
}

/// None of Err's forbidden values fit into Ok's unused bits, so branch on wherther
/// some of Ok's forbidden values fit into Err's forbidden value
///
/// If some forbidden values for Ok fit inside Err's unused bits, exit the recursion

impl<
        UnionSize: Unsigned,
        Budget,
        OkFv: IForbiddenValues,
        OkUb: IBitMask,
        ErrFv: IForbiddenValues,
        ErrUb: IBitMask,
        ErrSize: Unsigned,
        ErrAlign: PowerOf2,
        ErrOffset: Unsigned,
        Offset: Unsigned,
        V: Unsigned,
        Tail: IForbiddenValues + ISingleForbiddenValue + IntoValueIsErr,
        UbIntersect: IBitMask,
    > IDiscriminantProviderInner
    for DiscriminantProviderBranch<
        DiscriminantProvider<
            UnionSize,
            Budget,
            OkFv,
            OkUb,
            ErrFv,
            ErrUb,
            ErrSize,
            ErrAlign,
            ErrOffset,
        >,
        End,
        Array<Offset, V, Tail>,
        UbIntersect,
    >
{
    type ErrShift = ErrOffset;
    type Discriminant = <Array<Offset, V, Tail> as IntoValueIsErr>::ValueIsErr;
    type NicheExporter = NicheExporter<End, UbIntersect, Saturator>;
}

/// If neither Err nor Ok's unused bits can fit any of the other's forbidden value,
/// check if their unused bits have an intersection
///
/// If Ok and Err's unused bits have an intersection, use it.
impl<
        UnionSize: Unsigned,
        Budget,
        OkFv: IForbiddenValues,
        OkUb: IBitMask,
        ErrFv: IForbiddenValues,
        ErrUb: IBitMask,
        ErrSize: Unsigned,
        ErrAlign: PowerOf2,
        ErrOffset: Unsigned,
        Offset: Unsigned,
        V: NonZero,
        Tail: IBitMask,
    > IDiscriminantProviderInner
    for DiscriminantProviderBranch<
        DiscriminantProvider<
            UnionSize,
            Budget,
            OkFv,
            OkUb,
            ErrFv,
            ErrUb,
            ErrSize,
            ErrAlign,
            ErrOffset,
        >,
        End,
        End,
        Array<Offset, V, Tail>,
    >
{
    type ErrShift = ErrOffset;
    type Discriminant = BitIsErr<
        <Array<Offset, V, Tail> as IBitMask>::ExtractedBitByteOffset,
        <Array<Offset, V, Tail> as IBitMask>::ExtractedBitMask,
    >;
    type NicheExporter =
        NicheExporter<End, <Array<Offset, V, Tail> as IBitMask>::ExtractBit, Saturator>;
}
/// If no niche was found, check if Err can still be shifted to the right by its alignment.
impl<
        UnionSize: Unsigned,
        Budget,
        OkFv: IForbiddenValues,
        OkUb: IBitMask,
        ErrFv: IForbiddenValues,
        ErrUb: IBitMask,
        ErrSize: Unsigned,
        ErrAlign: PowerOf2,
        ErrOffset: Unsigned,
    > IDiscriminantProviderInner
    for DiscriminantProviderBranch<
        DiscriminantProvider<
            UnionSize,
            Budget,
            OkFv,
            OkUb,
            ErrFv,
            ErrUb,
            ErrSize,
            ErrAlign,
            ErrOffset,
        >,
        End,
        End,
        End,
    >
where
    (
        DiscriminantProvider<
            UnionSize,
            Budget,
            OkFv,
            OkUb,
            ErrFv,
            ErrUb,
            ErrSize,
            ErrAlign,
            ErrOffset,
        >,
        <tyeval!((ErrSize + ErrAlign) + ErrOffset) as Unsigned>::SmallerOrEq<UnionSize>,
    ): IDiscriminantProviderInner,
{
    same_as!((
        DiscriminantProvider<
            UnionSize,
            Budget,
            OkFv,
            OkUb,
            ErrFv,
            ErrUb,
            ErrSize,
            ErrAlign,
            ErrOffset,
        >,
        <tyeval!((ErrSize + ErrAlign) + ErrOffset) as Unsigned>::SmallerOrEq<UnionSize>
    ));
}
/// If it can't be shifted
impl<
        UnionSize: Unsigned,
        Budget,
        OkFv: IForbiddenValues,
        OkUb: IBitMask,
        ErrFv: IForbiddenValues,
        ErrUb: IBitMask,
        ErrSize: Unsigned,
        ErrAlign: PowerOf2,
        ErrOffset: Unsigned,
    > IDiscriminantProviderInner
    for (
        DiscriminantProvider<
            UnionSize,
            Budget,
            OkFv,
            OkUb,
            ErrFv,
            ErrUb,
            ErrSize,
            ErrAlign,
            ErrOffset,
        >,
        B0,
    )
{
    same_as!(DiscriminantProviderWithUnit<End, End>);
}
/// If it can be shifted

impl<
        UnionSize: Unsigned,
        Budget,
        OkFv: IForbiddenValues,
        OkUb: IBitMask,
        ErrFv: IForbiddenValues,
        ErrUb: IBitMask,
        ErrSize: Unsigned,
        ErrAlign: PowerOf2,
        ErrOffset: Unsigned,
    > IDiscriminantProviderInner
    for (
        DiscriminantProvider<
            UnionSize,
            T<Budget>,
            OkFv,
            OkUb,
            ErrFv,
            ErrUb,
            ErrSize,
            ErrAlign,
            ErrOffset,
        >,
        B1,
    )
where
    DiscriminantProvider<
        UnionSize,
        Budget,
        OkFv,
        OkUb,
        ErrFv,
        ErrUb,
        ErrSize,
        ErrAlign,
        tyeval!(ErrAlign + ErrOffset),
    >: IDiscriminantProviderInner,
{
    same_as!(DiscriminantProvider<
        UnionSize,
        Budget,
        OkFv,
        OkUb,
        ErrFv,
        ErrUb,
        ErrSize,
        ErrAlign,
        tyeval!(ErrAlign + ErrOffset),
    >);
}
