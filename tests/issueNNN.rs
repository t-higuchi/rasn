#[test]
#[allow(non_camel_case_types)]
fn test_ber_encoding_of_automatic_tagged_choices() {
    // With following ASN.1 definitions, BER encoding of TaggedChoice and TaggedReferencedChoice should be identical.
    //
    // TestModuleA DEFINITIONS AUTOMATIC TAGS::= BEGIN
    //     Choice1 ::= CHOICE { a INTEGER, b BOOLEAN }
    //
    //     TaggedReferencedChoice ::= [8] Choice1
    //     TaggedChoice           ::= [8] CHOICE { a INTEGER, b BOOLEAN }
    // END

    extern crate alloc;
    use rasn::prelude::*;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum Choice1 {
        a(Integer),
        b(bool),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, tag(explicit(context, 8)), automatic_tags)]
    pub enum TaggedChoice {
        a(Integer),
        b(bool),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, tag(explicit(context, 8)))]
    pub struct TaggedReferencedChoice(pub Choice1);

    let choice1 = Choice1::a(0x55.into());
    let tagged_ref_choice = TaggedReferencedChoice(choice1.clone());
    let tagged_choice = TaggedChoice::a(0x55.into());

    let choice_ber = rasn::ber::encode(&choice1).unwrap();
    let tagged_ref_choice_ber = rasn::ber::encode(&tagged_ref_choice).unwrap();
    let tagged_choice_ber = rasn::ber::encode(&tagged_choice).unwrap();

    assert_eq!(choice_ber, vec![0x80, 0x01, 0x55_u8]);
    assert_eq!(tagged_ref_choice_ber, vec![0xa8, 0x03, 0x80, 0x01, 0x55_u8]);
    assert_eq!(tagged_choice_ber, tagged_ref_choice_ber);

    // decode test
    let tagged_referenced_choice_de =
        rasn::ber::decode::<TaggedReferencedChoice>(&tagged_ref_choice_ber).unwrap();
    let tagged_choice_de = rasn::ber::decode::<TaggedChoice>(&tagged_choice_ber).unwrap();

    assert_eq!(tagged_ref_choice, tagged_referenced_choice_de);
    assert_eq!(tagged_choice, tagged_choice_de);
}
