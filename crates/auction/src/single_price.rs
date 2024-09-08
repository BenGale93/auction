//! Module containing the single price auction algorithm.
use crate::{Auction, Bid, Bids, Sale, Sales};

/// Resolves bids into sales using the single price algorithm.
///
/// # Arguments
/// * `auction` - The auction to resolve bids for.
/// * `bids` - The bids to resolve.
///
/// # Returns
/// A list of sales for the bids.
///
pub fn single_price(auction: &Auction, mut bids: Bids) -> Sales {
    bids.sort_by(|a, b| b.cmp(a));

    let mut remaining_lots = auction.lots;
    let mut winning_bids = Vec::new();
    for bid in bids.iter() {
        if bid.amount < auction.reserve_price {
            break;
        }
        if bid.quantity <= remaining_lots {
            remaining_lots -= bid.quantity;
            winning_bids.push(*bid);
        } else if remaining_lots > 0 {
            let new_bid = Bid::new(bid.amount, remaining_lots);
            winning_bids.push(new_bid);
            remaining_lots = 0;
        } else {
            break;
        }
    }

    let lowest_winning_bid_amount = match winning_bids.last() {
        None => return Vec::new(),
        Some(bid) => bid.amount,
    };

    winning_bids
        .iter()
        .map(|bid| Sale::new(bid.id, lowest_winning_bid_amount, bid.quantity))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn single_price_returns_empty_for_no_bids() {
        let bids: Bids = vec![];
        let auction = AuctionBuilder::new().lots(10).build();
        let sales = auction.resolve_bids(bids);
        assert!(sales.is_empty());
    }

    #[test]
    fn single_price_returns_all_bids_with_large_lot() {
        let bids: Bids = vec![bid![10, 1], bid![20, 1]];
        let bids_len = bids.len();
        let auction = AuctionBuilder::new().lots(3).build();
        let sales = auction.resolve_bids(bids);
        assert_eq!(sales.len(), bids_len);
        assert_eq!(sales[0].amount, 10);
    }

    #[test]
    fn single_price_return_some_of_bids_with_small_lot() {
        let bids: Bids = vec![bid![10, 1], bid![20, 1]];
        let auction = AuctionBuilder::new().lots(1).build();
        let sales = auction.resolve_bids(bids);
        assert_eq!(sales.len(), 1);
        assert_eq!(sales[0].amount, 20);
    }

    #[test]
    fn single_price_all_sales_have_same_amount() {
        let bids: Bids = vec![bid![10, 1], bid![20, 1]];
        let auction = AuctionBuilder::new().lots(2).build();
        let sales = auction.resolve_bids(bids);
        assert_eq!(sales.len(), 2);
        assert_eq!(sales[0].amount, 10);
        assert_eq!(sales[1].amount, 10);
    }

    #[test]
    fn single_price_partially_fulfilled() {
        let bids: Bids = vec![bid![10, 2], bid![20, 1]];
        let auction = AuctionBuilder::new().lots(2).build();
        let sales = auction.resolve_bids(bids);
        assert_eq!(sales.len(), 2);
        assert_eq!(sales[0].amount, 10);
        assert_eq!(sales[1].amount, 10);
        assert_eq!(sales[0].quantity, 1);
        assert_eq!(sales[1].quantity, 1);
    }

    #[test]
    fn single_price_reserve_price_applied() {
        let bids: Bids = vec![bid![55, 1], bid![20, 1]];
        let auction = AuctionBuilder::new().lots(2).reserve_price(50).build();
        let sales = auction.resolve_bids(bids);

        assert_eq!(sales.len(), 1);
        assert_eq!(sales[0].amount, 55);
    }
}
