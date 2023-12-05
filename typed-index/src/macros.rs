macro_rules! delegate {
    ($vis:vis fn $fn_ident:ident(self $(,$arg_ident:ident : $arg_ty:ty)* ) $(-> $ret_ty:ty)?) => {
        #[inline]
        $vis fn $fn_ident(self $(,$arg_ident: $arg_ty)*) $(-> $ret_ty)? {
            self.inner.$fn_ident($($arg_ident,)*)
        }
    };
    ($vis:vis fn $fn_ident:ident(&self $(,$arg_ident:ident : $arg_ty:ty)* ) $(-> $ret_ty:ty)?) => {
        #[inline]
        $vis fn $fn_ident(&self $(,$arg_ident: $arg_ty)*) $(-> $ret_ty)? {
            self.inner.$fn_ident($($arg_ident,)*)
        }
    };
    ($vis:vis fn $fn_ident:ident(&mut self $(,$arg_ident:ident : $arg_ty:ty)* ) $(-> $ret_ty:ty)?) => {
        #[inline]
        $vis fn $fn_ident(&mut self $(,$arg_ident: $arg_ty)*) $(-> $ret_ty)? {
            self.inner.$fn_ident($($arg_ident,)*)
        }
    };
}

pub(crate) use delegate;
