#[derive(Debug, PartialEq, Eq)]
pub struct DB<T> {
    data: Vec<T>,
}

/// An immutably borrowed subset of a DB
///
/// NB: (nota bene, or "take special note"): You will need to be explcit about the liftimes in this
/// struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBView<'a, T> {
    entries: Vec<&'a T>,
}

/// An mutably borrowed subset of a DB
///
/// NB: You will need to be explcit about the liftimes in this struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBViewMut<'a, T> {
    entries: Vec<&'a mut T>,
}

/// Filters a DBView using the the given predicate.
pub fn filter_one<T, F>(view: &DBView<T>, predicate: F) -> DBView< T>
    where F: Fn(&T) -> bool
{
    unimplemented!()
}

/// Filters two DBView structs using the same predicate, producing two separate results. This is
/// the moral equivalent of doing the two filters separately.
pub fn filter_two<T, F>(view_a: &DBView<T>,
                        view_b: &DBView<T>,
                        predicate: F)
                        -> (DBView<T>, DBView<T>)
    where F: Fn(&T) -> bool
{
    unimplemented!()
}

impl<T> DB<T> {
    /// Creates a DB from the given list of entries
    pub fn new(data: Vec<T>) -> DB<T> {
        DB{data: data}
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<'a, 'b, F>(&'a self, predicate: F) -> DBView<'b, T>
        where F: Fn(&T) -> bool,
              'a : 'b
    {
        let dbv = DBView{entries: Vec::new()};
        for x in &(self.data) {
            if predicate(x) {
                dbv.entries.push(x);
            }
        }
        dbv
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<'a, 'b, F>(&'a mut self, predicate: F) -> DBViewMut<'b, T>
        where F: Fn(&T) -> bool, 'a : 'b
    {
        let dbvm = DBViewMut{entries: Vec::new()};
        for x in &mut(self.data) {
            if predicate(x) {
                dbvm.entries.push(x);
            }
        }
        dbvm
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view(&self) -> DBView<T> {
        self.select_where(|_| true)
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view_mut(&mut self) -> DBViewMut<T> {
        self.select_where_mut(|_| true)
    }

    /// Returns the number of entries in the DB
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> DBView<T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<F>(&self, predicate: F) -> DBView<T>
        where F: Fn(&T) -> bool
    {
        unimplemented!()
    }

    /// Returns the number of entries in the DBView
    pub fn len(&self) -> usize {
        unimplemented!()
    }
}

impl<T> DBViewMut<T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<F>(self, predicate: F) -> DBViewMut<T>
        where F: Fn(&T) -> bool
    {
        unimplemented!()
    }

    /// Returns the number of entries in the DBView
    pub fn len(&self) -> usize {
        unimplemented!()
    }
}

// Bonus A
//
// impl<T> IntoIterator for DB<T> {
//     type Item = T;
//     // TODO
// }
//
// impl<T> IntoIterator for &DB<T> {
//     type Item = &T;
//     // TODO
// }
//
// impl<T> IntoIterator for &mut DB<T> {
//     type Item = &mut T;
//     // TODO
// }
//
// impl<T> IntoIterator for DBView<T> {
//     type Item = &T;
//     // TODO
// }
//
// impl<T> IntoIterator for DBViewMut<T> {
//     type Item = &mut T;
//     // TODO
// }
