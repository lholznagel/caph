use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::ops::Deref;

unsafe impl<T> Sync for LeftRight<T> where T: Clone + Default + Send {}

#[derive(Debug)]
pub struct LeftRight<T> {
    /// Contains the data for the left side
    left:  UnsafeCell<T>,
    /// Contains the data for the right side
    right: UnsafeCell<T>,

    /// Contains the currently active read side
    active_side:     Active,
    /// If the value is `true` the requesting writer has exclusive access
    /// to the datastructure. If the value is `false` there is another thread
    /// with exclusive write
    exclusive_write: AtomicBool,
    /// Counts the number of readers on the left side
    readers_left:    AtomicUsize,
    /// Counts the number of readers on the right side
    readers_right:   AtomicUsize,
}

impl<T> LeftRight<T> where T: Clone + Default + Send {
    pub fn new(data: T) -> Self {
        Self {
            left:  UnsafeCell::new(data.clone()),
            right: UnsafeCell::new(data),

            active_side:     Active::default(),
            exclusive_write: AtomicBool::new(true),
            readers_left:    AtomicUsize::default(),
            readers_right:   AtomicUsize::default(),
        }
    }
}

impl<T> Default for LeftRight<T> where T: Clone + Default + Send {
    fn default() -> Self {
        Self {
            left:  UnsafeCell::default(),
            right: UnsafeCell::default(),

            active_side:     Active::default(),
            exclusive_write: AtomicBool::new(true),
            readers_left:    AtomicUsize::default(),
            readers_right:   AtomicUsize::default(),
        }
    }
}

impl<T> LeftRight<T> where T: Clone + Default + Send {
    /// Takes a function and injects the current read side into the function
    ///
    /// # Example:
    /// ``` rust
    /// use caph_db::LeftRight;
    /// use std::collections::HashMap;
    /// 
    /// // Creates a new [LeftRight] instance, the [HashMap] can be replace
    /// // with any other type
    /// let left_right = LeftRight::<HashMap<u32, u32>>::default();
    /// // x contains the HashMap or any other configured type
    /// left_right.read(|x| {
    ///     let entry = x.get(&5);
    ///     dbg!(entry);
    /// })
    /// ```
    pub fn read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let active = self.arrive();

        // depending on which side is active, give it the reader
        let result = if active == ActiveSide::Left {
            unsafe { f(& (*self.left.get())) }
        } else {
            unsafe { f(& (*self.right.get())) }
        };

        self.depart(active);
        result
    }

    /// Takes a function and injects the current write side into the function
    ///
    /// The caller MUST call the [LeftRight::commit] function when the changes
    /// should be commited.
    ///
    /// # Example:
    /// ``` rust
    /// use caph_db::LeftRight;
    /// use std::collections::HashMap;
    /// 
    /// // Creates a new [LeftRight] instance, the [HashMap] can be replace
    /// // with any other type
    /// let left_right = LeftRight::<HashMap<u32, u32>>::default();
    /// // x contains the HashMap or any other configured type
    /// left_right.write(|x|
    ///     { x.entry(5).and_modify(|x| *x += 1).or_insert(0); }
    /// );
    /// ```
    pub fn write(&self, f: impl FnOnce(&mut T)) {
        // Make sure that we are the only ones that manipulate the structure
        while self
                .exclusive_write
                .compare_exchange_weak(
                    true, false,
                    Ordering::SeqCst, Ordering::SeqCst
                )
                .is_err() {}

        // When the left side is currently active, inject the right side.
        // When the right side is currently active, inject the left side.
        if self.active_side.get() == ActiveSide::Right {
            unsafe { f(&mut (*self.left.get())) }
        } else {
            unsafe { f(&mut (*self.right.get())) }
        }

        // Unlock datastructure
        self.exclusive_write.store(true, Ordering::SeqCst);
    }

    /// Commits all changes, switches sides and then updates the now unactive
    /// side
    pub fn commit(&self) {
        // Make sure that we are the only ones that manipulate the structure
        while self
                .exclusive_write
                .compare_exchange_weak(
                    true, false,
                    Ordering::SeqCst, Ordering::SeqCst
                )
                .is_err() {}

        let active_side = self.active_side.get();

        // switch the reader from left to right or from right to left
        self.active_side.swap(active_side);

        // Wait until all readers switched to the new side
        if active_side == ActiveSide::Left {
            while self.readers_left.load(Ordering::SeqCst) > 0 {  }
        } else {
            while self.readers_right.load(Ordering::SeqCst) > 0 {  }
        }

        // Finally copy the new data over to the other side
        if active_side == ActiveSide::Left {
            unsafe { *(self.left.get() as *mut _) = &mut *self.right.get() };
        } else {
            unsafe { *(self.right.get() as *mut _) = &mut *self.left.get() };
        }

        // Unlock datastructure
        self.exclusive_write.store(true, Ordering::SeqCst);
    }

    /// Depending on the active side, it increments the counter.
    ///
    /// Returns the used active side.
    fn arrive(&self) -> ActiveSide {
        if self.active_side.get() == ActiveSide::Left {
            self.readers_left.fetch_add(1, Ordering::SeqCst);
            ActiveSide::Left
        } else {
            self.readers_right.fetch_add(1, Ordering::SeqCst);
            ActiveSide::Right
        }
    }

    /// Decrements the counter for a side depending on the given active side.
    fn depart(&self, active: ActiveSide) {
        if active == ActiveSide::Left {
            self.readers_left.fetch_sub(1, Ordering::SeqCst);
        } else {
            self.readers_right.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

/// Wrapper for atomic bool that represents the current active side.
#[derive(Debug, Default)]
struct Active(AtomicBool);

impl Active {
    /// Gets the current active side
    pub fn get(&self) -> ActiveSide {
        ActiveSide::from(self.0.load(Ordering::SeqCst))
    }

    /// Swaps [ActiveSide::Left] to [ActiveSide::Right] and
    /// [Active::Right] to [Active::Left]
    pub fn swap(&self, active: ActiveSide) {
        self.0.store(!*active, Ordering::SeqCst);
    }
}

/// Represents the active side
#[derive(Copy, Clone, Debug, PartialEq)]
enum ActiveSide {
    Left,
    Right,
}

impl From<bool> for ActiveSide {
    fn from(x: bool) -> Self {
        match x {
            true  => Self::Left,
            false => Self::Right,
        }
    }
}

impl Deref for ActiveSide {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Left  => &true,
            Self::Right => &false,
        }
    }
}
