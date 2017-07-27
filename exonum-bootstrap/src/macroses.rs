#[macro_export]
macro_rules! transactions {
    (
        $($id:ident => $ty:ty)*
    ) => (
        $(
            #[transaction]
            #[ty="$ty"]
            #[id="$id"]
            _$ty: bool
        )*
    )
}

#[macro_export]
macro_rules! key {
    ($name:ident: $ty:ty) => (
        field! {
            #[key]
            $name: $ty
        }
    )
}
#[macro_export]
macro_rules! set {
    ($name:ident: $ty:ty) => (
        field! {
            #[set]
            $name: $ty
        }
    )
}

#[macro_export]
macro_rules! get {
    (
        $(#[$field_attr:meta])*
        $name:ident: $ty:ty
    ) => (
        $(#[$attr])*
        get!($name: $ty)
    );
    ($name:ident: &PublicKey) => (
        #[size="32"]
        #[ty="&PublicKey"]
        $name: bool
    );
    ($name:ident: u8) => (
        #[size="2"]
        #[ty="u8"]
        $name: bool
    );
    ($name:ident: u16) => (
        #[size="4"]
        #[ty="u16"]
        $name: bool
    );
    ($name:ident: u32) => (
        #[size="4"]
        #[ty="u32"]
        $name: bool
    );
    ($name:ident: u64) => (
        #[size="8"]
        #[ty="u64"]
        $name: bool
    );
    ($name:ident: i8) => (
        #[size="2"]
        #[ty="i8"]
        $name: bool
    );
    ($name:ident: i16) => (
        #[size="4"]
        #[ty="i16"]
        $name: bool
    );
    ($name:ident: i32) => (
        #[size="4"]
        #[ty="i32"]
        $name: bool
    );
    ($name:ident: i64) => (
        #[size="8"]
        #[ty="i64"]
        $name: bool
    );
    ($name:ident: $ty:ty, $size:expr) => {
        #[size="$size"]
        #[ty="$ty"]
        $name: bool
    }
}

#[macro_export]
macro_rules! record_struct {
    (
        SERVICE = $service:expr;
        RECORD_ID = $id:expr;
        struct $name:ident {
            key($key_name:ident: $key_ty:ty)
            
            // $(get($getter_name:ident: bool))*
            // $(get($getter_name:ident: u8))*
            // $(get($getter_name:ident: u16))*
            // $(get($getter_name:ident: u32))*
            // $(get($getter_name:ident: u64))*
            // $(get($getter_name:ident: i8))*
            // $(get($getter_name:ident: i16))*
            // $(get($getter_name:ident: i32))*
            // $(get($getter_name:ident: i64))*
            $(
                get($getter_name:ident: $getter_ty:ty [$getter_size:expr])
            )*
            // $(set($setter_name:ident: bool))*
            // $(set($setter_name:ident: u8))*
            // $(set($setter_name:ident: u16))*
            // $(set($setter_name:ident: u32))*
            // $(set($setter_name:ident: u64))*
            // $(set($setter_name:ident: i8))*
            // $(set($setter_name:ident: i16))*
            // $(set($setter_name:ident: i32))*
            // $(set($setter_name:ident: i64))*
            $(
                set($setter_name:ident: $setter_ty:ty [$setter_size:expr])
            )*

            $(
                transaction($t_ty:ident, $t_id:expr)
            )*
        }
    ) => {
        #[derive(exonum_record)]
        #[service = "$service"]
        #[id = "$id"]
        pub struct $name {
            #[key]
            #[size="32"]
            #[ty="$key_ty"]
            $key_name: bool,

                // $(
				// 	#[size="1"]
				// 	#[ty="bool"]
				// 	$getter_name: bool
				// )*
                // $(
				// 	#[size="1"]
				// 	#[ty="u8"]
				// 	$getter_name: bool
				// )*
                // $(
				// 	#[size="2"]
				// 	#[ty="u16"]
				// 	$getter_name: bool
				// )*
                // $(
				// 	#[size="4"]
				// 	#[ty="u32"]
				// 	$getter_name: bool
				// )*
                // $(
				// 	#[size="8"]
				// 	#[ty="u64"]
				// 	$getter_name: bool
				// )*
                // $(
				// 	#[size="1"]
				// 	#[ty="i8"]
				// 	$getter_name: bool
				// )*
                // $(
				// 	#[size="2"]
				// 	#[ty="i16"]
				// 	$getter_name: bool
				// )*
                // $(
				// 	#[size="4"]
				// 	#[ty="i32"]
				// 	$getter_name: bool
				// )*
                // $(
				// 	#[size="8"]
				// 	#[ty="i64"]
				// 	$getter_name: bool
				// )*

                $(
					#[size="$getter_size"]
					#[ty="$getter_ty"]
					$getter_name: bool,
				)*
                // $(
                //     #[set]
				// 	#[size="1"]
				// 	#[ty="bool"]
				// 	$setter_name: bool
				// )*
                // $(
                //     #[set]
				// 	#[size="1"]
				// 	#[ty="u8"]
				// 	$setter_name: bool
				// )*
                // $(
                //     #[set]
				// 	#[size="2"]
				// 	#[ty="u16"]
				// 	$setter_name: bool
				// )*
                // $(
                //     #[set]
				// 	#[size="4"]
				// 	#[ty="u32"]
				// 	$setter_name: bool
				// )*
                // $(
                //     #[set]
				// 	#[size="8"]
				// 	#[ty="u64"]
				// 	$setter_name: bool
				// )*
                // $(
                //     #[set]
				// 	#[size="1"]
				// 	#[ty="i8"]
				// 	$setter_name: bool
				// )*
                // $(
                //     #[set]
				// 	#[size="2"]
				// 	#[ty="i16"]
				// 	$setter_name: bool
				// )*
                // $(
                //     #[set]
				// 	#[size="4"]
				// 	#[ty="i32"]
				// 	$setter_name: bool
				// )*
                // $(
                //     #[set]
				// 	#[size="8"]
				// 	#[ty="i64"]
				// 	$setter_name: bool
				// )*
                
                $(
                    #[set]
					#[size="$getter_size"]
					#[ty="$getter_ty"]
					$getter_name: bool,
				)*

            $(
                #[transaction]
                #[id="$t_id"]
                #[ty="$t_ty"]
                $t_ty: bool,
            )*
        }
    }
}