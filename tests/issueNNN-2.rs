#[test]
fn test_tagged_tagged_choice_round_trip() {
    /*
    TestModuleA DEFINITIONS AUTOMATIC TAGS::= BEGIN
        --untagged choice
        C1 ::= CHOICE { a INTEGER, b BOOLEAN }

        --tagged choice. This is explicit tagging. (see ITU-T X.680 section 31.2.7 clause c)
        TC1  ::= [3] C1
        --another form of tagged choice
        TC2  ::= [4] CHOICE { a INTEGER, b BOOLEAN }

        --tagged tagged choice 1 (implicit tagging)
        TTC1 ::= [5] TC1
        --tagged tagged choice 2 (implicit tagging)
        TTC2 ::= [6] TC2
    END
    */

    use rasn::prelude::*;
    #[doc = "untagged choice"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum C1 {
        A(Integer),
        B(bool),
    }
    #[doc = "tagged choice. This is explicit tagging. (see ITU-T X.680 section 31.2.7 clause c)"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, tag(explicit(context, 3)))] // explicit since C1 is untagged choice
    pub struct TC1(pub C1);
    #[doc = "another form of tagged choice"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, tag(explicit(context, 4)), automatic_tags)] // explicit
    pub enum TC2 {
        A(Integer),
        B(bool),
    }
    #[doc = "tagged tagged choice 1 (implicit tagging)"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, tag(context, 5))]
    pub struct TTC1(pub TC1);
    #[doc = "tagged tagged choice 2 (implicit tagging)"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, tag(context, 6))]
    pub struct TTC2(pub TC2);

    let choice = C1::A(0x55.into());
    let tc1 = TC1(choice.clone());
    let tc2 = TC2::A(0x55.into());
    let ttc1 = TTC1(tc1.clone());
    let ttc2 = TTC2(tc2.clone());

    let choice_enc = rasn::ber::encode(&choice).unwrap();
    let tc1_enc = rasn::ber::encode(&tc1).unwrap();
    let tc2_enc = rasn::ber::encode(&tc2).unwrap();
    let ttc1_enc = rasn::ber::encode(&ttc1).unwrap();
    let ttc2_enc = rasn::ber::encode(&ttc2).unwrap();

    assert_eq!(choice_enc, vec![0x80, 0x01, 0x55]);
    assert_eq!(tc1_enc, vec![0xa3, 0x03, 0x80, 0x01, 0x55]);
    assert_eq!(tc2_enc, vec![0xa4, 0x03, 0x80, 0x01, 0x55]);
    assert_eq!(ttc1_enc, vec![0xa5, 0x03, 0x80, 0x01, 0x55]);
    assert_eq!(ttc2_enc, vec![0xa6, 0x03, 0x80, 0x01, 0x55]);

    let tc1_de = rasn::ber::decode::<TC1>(&tc1_enc).unwrap();
    let tc2_de = rasn::ber::decode::<TC2>(&tc2_enc).unwrap();
    let ttc1_de = rasn::ber::decode::<TTC1>(&ttc1_enc).unwrap();
    let ttc2_de = rasn::ber::decode::<TTC2>(&ttc2_enc).unwrap();
    assert_eq!(tc1, tc1_de);
    assert_eq!(tc2, tc2_de);
    assert_eq!(ttc1, ttc1_de);
    assert_eq!(ttc2, ttc2_de);
}
