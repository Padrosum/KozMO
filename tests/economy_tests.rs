use dominion_sim::{
    LedgerOperation, ResourceExpenditure, ResourceLedger, ResourceRemovalReason, ResourceSource,
    Resources,
};

#[test]
fn test_resource_source_expenditure_transfer_and_removal_tracking() {
    let mut ledger = ResourceLedger::new(64);
    let mut player_res = Resources::ZERO;
    let mut rival_res = Resources::new(200, 100, 50);

    // 1. Track Source
    ledger.record_source(
        1,
        &mut player_res,
        0,
        ResourceSource::MortalDeaths,
        Resources::new(150, 0, 0),
        Some(0),
        "Ölümlü savaş kayıpları",
    );
    ledger.record_source(
        1,
        &mut player_res,
        0,
        ResourceSource::FaithDevotion,
        Resources::new(80, 45, 5),
        Some(0),
        "Tapınak ibadetleri",
    );

    assert_eq!(player_res, Resources::new(230, 45, 5));
    assert_eq!(
        ledger.totals_by_source.get(&ResourceSource::MortalDeaths),
        Some(&Resources::new(150, 0, 0))
    );
    assert_eq!(
        ledger.totals_by_source.get(&ResourceSource::FaithDevotion),
        Some(&Resources::new(80, 45, 5))
    );

    // 2. Track Expenditure
    let ok = ledger.try_record_expenditure(
        2,
        &mut player_res,
        0,
        ResourceExpenditure::DivineAction,
        Resources::new(35, 15, 0),
        Some(0),
        "Kehanet yayılımı",
    );
    assert!(ok);
    assert_eq!(player_res, Resources::new(195, 30, 5));

    // Unaffordable expenditure should fail without mutating resources
    let fail = ledger.try_record_expenditure(
        2,
        &mut player_res,
        0,
        ResourceExpenditure::PowerUnlock,
        Resources::new(500, 100, 50),
        None,
        "Ağır güç",
    );
    assert!(!fail);
    assert_eq!(player_res, Resources::new(195, 30, 5));

    // 3. Track Transfer between deities
    let transferred = ledger.record_transfer(
        3,
        &mut rival_res,
        1,
        &mut player_res,
        0,
        Resources::new(60, 20, 10),
        "Diplomatik haraç",
    );
    assert_eq!(transferred, Resources::new(60, 20, 10));
    assert_eq!(rival_res, Resources::new(140, 80, 40));
    assert_eq!(player_res, Resources::new(255, 50, 15));
    assert_eq!(ledger.total_transferred, Resources::new(60, 20, 10));

    // 4. Track Removal / Decay
    let removed = ledger.record_removal(
        4,
        &mut player_res,
        0,
        ResourceRemovalReason::UniverseCollapse,
        Resources::new(25, 0, 0),
        Some(3),
        "Çöken evren sızıntısı",
    );
    assert_eq!(removed, Resources::new(25, 0, 0));
    assert_eq!(player_res, Resources::new(230, 50, 15));
    assert_eq!(
        ledger
            .totals_by_removal
            .get(&ResourceRemovalReason::UniverseCollapse),
        Some(&Resources::new(25, 0, 0))
    );
    assert!(ledger
        .recent_entries
        .iter()
        .any(|e| matches!(e.operation, LedgerOperation::Removal(_))));
}
