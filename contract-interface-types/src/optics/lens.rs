use std::marker::PhantomData;

pub trait Lens<Inner> {
    fn with_ref<V, F>(&self, f: F) -> V
    where
        F: FnOnce(&Inner) -> V;

    fn with_mut<V, F>(&mut self, f: F) -> V
    where
        F: FnOnce(&mut Inner) -> V;
}

pub trait Lens3<Outer, Inner> {
    fn with_ref<V, F>(data: &Outer, f: F) -> V
    where
        F: FnOnce(&Inner) -> V;

    fn with_mut<V, F>(data: &mut Outer, f: F) -> V
    where
        F: FnOnce(&mut Inner) -> V;
}

// pub trait Lens<T1: ?Sized, T2: ?Sized> {
//     fn with_ref<V, F>(&self, data: &T1, f: F) -> V
//     where
//         F: FnOnce(&T2) -> V;

//     fn with_mut<V, F>(&self, data: &mut T1, f: F) -> V
//     where
//         F: FnOnce(&mut T2) -> V;
// }

pub trait Lens2<T1, T2> {
    fn with_ref<V, F>(&self, data: &T1, f: F) -> V
    where
        F: FnOnce(&T2) -> V;

    fn with_mut<V, F>(&self, data: &mut T1, f: F) -> V
    where
        F: FnOnce(&mut T2) -> V;

    fn with_owned<F>(&self, data: T1, f: F) -> T1
    where
        F: FnOnce(T2) -> T2;
}

#[derive(Default)]
pub struct Identity;

impl<T> Lens3<T, T> for Identity {
    fn with_ref<V, F>(data: &T, f: F) -> V
    where
        F: FnOnce(&T) -> V,
    {
        f(data)
    }

    fn with_mut<V, F>(data: &mut T, f: F) -> V
    where
        F: FnOnce(&mut T) -> V,
    {
        f(data)
    }
}

#[derive(Default)]
pub struct NonLensing;

#[derive(Default)]
pub struct Final;

#[derive(Default)]
pub struct Forbidden;

#[derive(Default)]
pub struct Then<L1, L2, T2> {
    _l1: PhantomData<L1>,
    _l2: PhantomData<L2>,
    _t2: PhantomData<T2>,
}

impl<L1, L2, T1, T2, T3> Lens3<T1, T3> for Then<L1, L2, T2>
where
    L1: Lens3<T1, T2>,
    L2: Lens3<T2, T3>,
{
    fn with_ref<V, F>(data: &T1, f: F) -> V
    where
        F: FnOnce(&T3) -> V,
    {
        L1::with_ref(data, |t2| L2::with_ref(t2, f))
    }

    fn with_mut<V, F>(data: &mut T1, f: F) -> V
    where
        F: FnOnce(&mut T3) -> V,
    {
        L1::with_mut(data, |t2| L2::with_mut(t2, f))
    }
}

pub trait IThen<L1, L2, T2> {
    fn then() -> Then<L1, L2, T2>;
}

impl<L1, L2, T2> IThen<L1, L2, T2> for L1 {
    fn then() -> Then<L1, L2, T2> {
        Then {
            _l1: PhantomData,
            _l2: PhantomData,
            _t2: PhantomData,
        }
    }
}

// pub struct Then<L1, L2, T1, T2> {
//     left: L1,
//     _right: PhantomData<L2>,
//     _marker1: PhantomData<T1>,
//     _marker2: PhantomData<T2>,
// }

// impl<T3, L> Lens<T3> for L {

// }

// impl<L1, L2, T1, T2, T3> Lens<T3> for Then<L1, L2, T1, T2>
// where
//     L1: Lens<T2>,
//     L2: Lens<T3>,
// {
//     fn with_ref<V, F>(&self, f: F) -> V
//     where
//         F: FnOnce(&T3) -> V,
//     {
//         let a = |t2: &T2| ;
//         let l2 = self.with_ref(f);
//         // let t2f = |t2: &T2| self.right.with(t2, f);
//         // self.left.with_ref(data, t2f)
//         todo!()
//     }

//     fn with_mut<V, F>(&mut self, f: F) -> V
//     where
//         F: FnOnce(&mut T3) -> V,
//     {
//         todo!()
//     }
// }

/*
impl<L1, L2, T1, T2, T3> Lens<T1, T3> for Then<L1, L2, T2>
where
    T1: ?Sized,
    T2: ?Sized,
    T3: ?Sized,
    L1: Lens<T1, T2>,
    L2: Lens<T2, T3>,
{
    fn with_ref<V, F>(&self, data: &T1, f: F) -> V
    where
        F: FnOnce(&T3) -> V,
    {
        let t2f = |t2: &T2| self.right.with(t2, f);
        self.left.with_ref(data, t2f)
    }

    fn with_mut<V, F>(&self, data: &mut T1, f: F) -> V
    where
        F: FnOnce(&mut T3) -> V,
    {
        self.left.with_mut(data, |t2| self.right.with_mut(t2, f))
    }
}
*/
