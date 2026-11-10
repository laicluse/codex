use super::*;

#[test]
fn completes_only_after_both_colors_arrive_in_either_order() {
    for slots in [[10, 11], [11, 10]] {
        let mut refresh = PaletteRefresh::default();
        refresh.begin();
        refresh.receive(slots[0], (1, 2, 3));
        assert_eq!(refresh.take_completed(), None);
        refresh.receive(slots[1], (4, 5, 6));
        refresh.finish();
        let expected = if slots[0] == 10 {
            DefaultColors {
                fg: (1, 2, 3),
                bg: (4, 5, 6),
            }
        } else {
            DefaultColors {
                fg: (4, 5, 6),
                bg: (1, 2, 3),
            }
        };
        assert_eq!(refresh.take_completed(), Some(expected));
    }
}

#[test]
fn a_new_query_can_succeed_after_an_incomplete_response() {
    let mut refresh = PaletteRefresh::default();
    refresh.begin();
    refresh.receive(10, (1, 2, 3));
    refresh.finish();
    assert_eq!(refresh.take_completed(), None);
    refresh.begin();
    refresh.receive(11, (7, 8, 9));
    refresh.receive(10, (10, 11, 12));
    refresh.finish();
    assert_eq!(
        refresh.take_completed(),
        Some(DefaultColors {
            fg: (10, 11, 12),
            bg: (7, 8, 9)
        })
    );
}

#[test]
fn notifications_during_a_query_refresh_again_after_its_response_fence() {
    let mut refresh = PaletteRefresh::default();
    assert!(refresh.begin());
    refresh.receive(10, (1, 1, 1));
    assert!(!refresh.begin());
    refresh.receive(11, (2, 2, 2));
    assert!(refresh.finish());
    assert_eq!(refresh.take_completed(), None);
    assert!(refresh.begin());
    refresh.receive(11, (3, 3, 3));
    refresh.receive(10, (4, 4, 4));
    assert!(!refresh.finish());
    assert_eq!(
        refresh.take_completed(),
        Some(DefaultColors {
            fg: (4, 4, 4),
            bg: (3, 3, 3)
        })
    );
}
