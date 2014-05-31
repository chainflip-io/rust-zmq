//! Module: zmq

#![crate_id = "zmq#0.5-pre"]

#![license = "MIT/ASL2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(phase, macro_rules)]

#[phase(syntax, link)]
extern crate log;
extern crate libc;

use libc::{c_int, c_long, c_void, size_t, c_char, int64_t, uint64_t};
use libc::consts::os::posix88;
use std::{mem, ptr, str, slice};
use std::fmt;

/// The ZMQ container that manages all the sockets
type Context_ = *c_void;

/// A ZMQ socket
type Socket_ = *c_void;

/// A message
type Msg_ = [c_char, ..32];

#[link(name = "zmq")]
extern {
    fn zmq_version(major: *c_int, minor: *c_int, patch: *c_int);

    fn zmq_ctx_new() -> Context_;
    fn zmq_ctx_destroy(ctx: Context_) -> c_int;

    fn zmq_errno() -> c_int;
    fn zmq_strerror(errnum: c_int) -> *c_char;

    fn zmq_socket(ctx: Context_, typ: c_int) -> Socket_;
    fn zmq_close(socket: Socket_) -> c_int;

    fn zmq_getsockopt(socket: Socket_, opt: c_int, optval: *mut c_void, size: *mut size_t) -> c_int;
    fn zmq_setsockopt(socket: Socket_, opt: c_int, optval: *c_void, size: size_t) -> c_int;

    fn zmq_bind(socket: Socket_, endpoint: *c_char) -> c_int;
    fn zmq_connect(socket: Socket_, endpoint: *c_char) -> c_int;

    fn zmq_msg_init(msg: &Msg_) -> c_int;
    fn zmq_msg_init_size(msg: &Msg_, size: size_t) -> c_int;
    fn zmq_msg_data(msg: &Msg_) -> *u8;
    fn zmq_msg_size(msg: &Msg_) -> size_t;
    fn zmq_msg_close(msg: &Msg_) -> c_int;

    fn zmq_msg_send(msg: &Msg_, socket: Socket_, flags: c_int) -> c_int;
    fn zmq_msg_recv(msg: &Msg_, socket: Socket_, flags: c_int) -> c_int;

    fn zmq_poll(items: *mut PollItem, nitems: c_int, timeout: c_long) -> c_int;
}

/// Socket types
#[allow(non_camel_case_types)]
#[deriving(Clone, Show)]
pub enum SocketType {
    PAIR   = 0,
    PUB    = 1,
    SUB    = 2,
    REQ    = 3,
    REP    = 4,
    DEALER = 5,
    ROUTER = 6,
    PULL   = 7,
    PUSH   = 8,
    XPUB   = 9,
    XSUB   = 10,
}

pub static DONTWAIT : int = 1;
pub static SNDMORE : int = 2;

#[allow(non_camel_case_types)]
#[deriving(Clone)]
#[allow(non_camel_case_types)]
pub enum Constants {
    ZMQ_AFFINITY          = 4,
    ZMQ_IDENTITY          = 5,
    ZMQ_SUBSCRIBE         = 6,
    ZMQ_UNSUBSCRIBE       = 7,
    ZMQ_RATE              = 8,
    ZMQ_RECOVERY_IVL      = 9,
    ZMQ_MCAST_LOOP        = 10,
    ZMQ_SNDBUF            = 11,
    ZMQ_RCVBUF            = 12,
    ZMQ_RCVMORE           = 13,
    ZMQ_FD                = 14,
    ZMQ_EVENTS            = 15,
    ZMQ_TYPE              = 16,
    ZMQ_LINGER            = 17,
    ZMQ_RECONNECT_IVL     = 18,
    ZMQ_BACKLOG           = 19,
    ZMQ_RECOVERY_IVL_MSEC = 20,
    ZMQ_RECONNECT_IVL_MAX = 21,
    ZMQ_MAXMSGSIZE        = 22,
    ZMQ_SNDHWM            = 23,
    ZMQ_RCVHWM            = 24,

    ZMQ_MAX_VSM_SIZE      = 30,
    ZMQ_DELIMITER         = 31,
    ZMQ_VSM               = 32,

    ZMQ_MSG_MORE          = 1,
    ZMQ_MSG_SHARED        = 128,
    ZMQ_MSG_MASK          = 129,

    ZMQ_HAUSNUMERO        = 156384712,
}

impl Constants {
    pub fn to_raw(&self) -> i32 {
        *self as i32
    }

    pub fn from_raw(raw: i32) -> Constants {
        // fails if `raw` is not a valid value
        match raw {
            4         => ZMQ_AFFINITY,
            5         => ZMQ_IDENTITY,
            6         => ZMQ_SUBSCRIBE,
            7         => ZMQ_UNSUBSCRIBE,
            8         => ZMQ_RATE,
            9         => ZMQ_RECOVERY_IVL,
            10        => ZMQ_MCAST_LOOP,
            11        => ZMQ_SNDBUF,
            12        => ZMQ_RCVBUF,
            13        => ZMQ_RCVMORE,
            14        => ZMQ_FD,
            15        => ZMQ_EVENTS,
            16        => ZMQ_TYPE,
            17        => ZMQ_LINGER,
            18        => ZMQ_RECONNECT_IVL,
            19        => ZMQ_BACKLOG,
            20        => ZMQ_RECOVERY_IVL_MSEC,
            21        => ZMQ_RECONNECT_IVL_MAX,
            22        => ZMQ_MAXMSGSIZE,
            23        => ZMQ_SNDHWM,
            24        => ZMQ_RCVHWM,

            30        => ZMQ_MAX_VSM_SIZE,
            31        => ZMQ_DELIMITER,
            32        => ZMQ_VSM,

            1         => ZMQ_MSG_MORE,
            128       => ZMQ_MSG_SHARED,
            129       => ZMQ_MSG_MASK,

            156384712 => ZMQ_HAUSNUMERO,

            x         => fail!("invalid constant {}", x as int),
        }
    }
}

#[deriving(Clone, Eq, PartialEq)]
pub enum Error {
    EACCES          = posix88::EACCES,
    EADDRINUSE      = posix88::EADDRINUSE,
    EAGAIN          = posix88::EAGAIN,
    EBUSY           = posix88::EBUSY,
    ECONNREFUSED    = posix88::ECONNREFUSED,
    EFAULT          = posix88::EFAULT,
    EHOSTUNREACH    = posix88::EHOSTUNREACH,
    EINPROGRESS     = posix88::EINPROGRESS,
    EINVAL          = posix88::EINVAL,
    EMFILE          = posix88::EMFILE,
    EMSGSIZE        = posix88::EMSGSIZE,
    ENAMETOOLONG    = posix88::ENAMETOOLONG,
    ENODEV          = posix88::ENODEV,
    ENOENT          = posix88::ENOENT,
    ENOMEM          = posix88::ENOMEM,
    ENOTCONN        = posix88::ENOTCONN,
    ENOTSOCK        = posix88::ENOTSOCK,
    EPROTO          = posix88::EPROTO,
    EPROTONOSUPPORT = posix88::EPROTONOSUPPORT,
    // magic number is EHAUSNUMERO + num
    ENOTSUP         = 156384713,
    ENOBUFS         = 156384715,
    ENETDOWN        = 156384716,
    EADDRNOTAVAIL   = 156384718,

    // native zmq error codes
    EFSM            = 156384763,
    ENOCOMPATPROTO  = 156384764,
    ETERM           = 156384765,
    EMTHREAD        = 156384766,
}

impl Error {
    pub fn to_raw(&self) -> i32 {
        *self as i32
    }

    pub fn from_raw(raw: i32) -> Error {
        match raw {
            posix88::EACCES          => EACCES,
            posix88::EADDRINUSE      => EADDRINUSE,
            posix88::EAGAIN          => EAGAIN,
            posix88::EBUSY           => EBUSY,
            posix88::ECONNREFUSED    => ECONNREFUSED,
            posix88::EFAULT          => EFAULT,
            posix88::EHOSTUNREACH    => EHOSTUNREACH,
            posix88::EINPROGRESS     => EINPROGRESS,
            posix88::EINVAL          => EINVAL,
            posix88::EMFILE          => EMFILE,
            posix88::EMSGSIZE        => EMSGSIZE,
            posix88::ENAMETOOLONG    => ENAMETOOLONG,
            posix88::ENODEV          => ENODEV,
            posix88::ENOENT          => ENOENT,
            posix88::ENOMEM          => ENOMEM,
            posix88::ENOTCONN        => ENOTCONN,
            posix88::ENOTSOCK        => ENOTSOCK,
            posix88::EPROTO          => EPROTO,
            posix88::EPROTONOSUPPORT => EPROTONOSUPPORT,
            156384713             => ENOTSUP,
            156384714             => EPROTONOSUPPORT,
            156384715             => ENOBUFS,
            156384716             => ENETDOWN,
            156384717             => EADDRINUSE,
            156384718             => EADDRNOTAVAIL,
            156384719             => ECONNREFUSED,
            156384720             => EINPROGRESS,
            156384721             => ENOTSOCK,
            156384763             => EFSM,
            156384764             => ENOCOMPATPROTO,
            156384765             => ETERM,
            156384766             => EMTHREAD,

            x => {
                unsafe {
                    fail!("unknown error [{}]: {}",
                          x as int,
                          str::raw::from_c_str(zmq_strerror(x))
                    )
                }
            }
        }
    }
}

// Return the current zeromq version.
pub fn version() -> (int, int, int) {
    let major = 0;
    let minor = 0;
    let patch = 0;

    unsafe {
        zmq_version(&major, &minor, &patch);
    }

    (major as int, minor as int, patch as int)
}

/// zmq context, used to create sockets. Is thread safe, and can be safely
/// shared, but dropping it while sockets are still open will cause them to
/// close (see zmq_ctx_destroy(3)).
///
/// For this reason, one should use an Arc to share it, rather than any unsafe
/// trickery you might think up that would call the destructor.
pub struct Context {
    ctx: Context_,
}

impl Context {
    pub fn new() -> Context {
        Context {
            ctx: unsafe { zmq_ctx_new() }
        }
    }

    pub fn socket(&mut self, socket_type: SocketType) -> Result<Socket, Error> {
        let sock = unsafe {zmq_socket(self.ctx, socket_type as c_int)};

        if sock.is_null() {
            return Err(errno_to_error());
        }

        Ok(Socket { sock: sock, closed: false })
    }

    /// Try to destroy the context. This is different than the destructor; the
    /// destructor will loop when zmq_ctx_destroy returns EINTR
    pub fn destroy(&mut self) -> Result<(), Error> {
        if unsafe { zmq_ctx_destroy(self.ctx) } == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        debug!("context dropped");
        let mut e = self.destroy();
        while e.is_err() && (e.unwrap_err() != EFAULT) {
            e = self.destroy();
        }
    }
}

pub struct Socket {
    sock: Socket_,
    closed: bool
}

impl Drop for Socket {
    fn drop(&mut self) {
        match self.close_final() {
            Ok(()) => { debug!("socket dropped") },
            Err(e) => fail!(e.to_str())
        }
    }
}

impl Socket {
    /// Accept connections on a socket.
    pub fn bind(&mut self, endpoint: &str) -> Result<(), Error> {
        let rc = endpoint.with_c_str (|cstr| {
            unsafe {zmq_bind(self.sock, cstr)}
        });

        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }

    /// Connect a socket.
    pub fn connect(&mut self, endpoint: &str) -> Result<(), Error> {
        let rc = endpoint.with_c_str (|cstr| {
            unsafe {zmq_connect(self.sock, cstr)}
        });

        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }

    /// Send a message
    pub fn send(&mut self, data: &[u8], flags: int) -> Result<(), Error> {
        unsafe {
            let base_ptr = data.as_ptr();
            let len = data.len();
            let msg = [0, ..32];

            // Copy the data into the message.
            let rc = zmq_msg_init_size(&msg, len as size_t);

            if rc == -1i32 { return Err(errno_to_error()); }

            ptr::copy_memory(zmq_msg_data(&msg) as *mut u8, base_ptr, len);

            let rc = zmq_msg_send(&msg, self.sock, flags as c_int);
            let _ = zmq_msg_close(&msg);

            if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
        }
    }

    pub fn send_str(&mut self, data: &str, flags: int) -> Result<(), Error> {
        self.send(data.as_bytes(), flags)
    }

    /// Receive a message into a `Message`. The length passed to zmq_msg_recv
    /// is the length of the buffer.
    pub fn recv(&mut self, msg: &mut Message, flags: int) -> Result<(), Error> {
        let rc = unsafe {
            zmq_msg_recv(&msg.msg, self.sock, flags as c_int)
        };

        if rc == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }

    pub fn recv_msg(&mut self, flags: int) -> Result<Message, Error> {
        let mut msg = Message::new();
        match self.recv(&mut msg, flags) {
            Ok(()) => Ok(msg),
            Err(e) => Err(e),
        }
    }

    pub fn recv_bytes(&mut self, flags: int) -> Result<~[u8], Error> {
        match self.recv_msg(flags) {
            Ok(msg) => Ok(msg.to_bytes()),
            Err(e) => Err(e),
        }
    }

    pub fn recv_str(&mut self, flags: int) -> Result<String, Error> {
        match self.recv_msg(flags) {
            Ok(msg) => Ok(msg.to_str()),
            Err(e) => Err(e),
        }
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if !self.closed {
            self.closed = true;

            if unsafe { zmq_close(self.sock) } == -1i32 {
                return Err(errno_to_error());
            }
        }

        Ok(())
    }

    pub fn close_final(&mut self) -> Result<(), Error> {
        if !self.closed {
            if unsafe { zmq_close(self.sock) } == -1i32 {
                return Err(errno_to_error());
            }
        }

        Ok(())
    }

    pub fn get_socket_type(&self) -> Result<SocketType, Error> {
        getsockopt_int(self.sock, ZMQ_TYPE.to_raw()).map(|ty| {
            match ty {
                0 => PAIR,
                1 => PUB,
                2 => SUB,
                3 => REQ,
                4 => REP,
                5 => DEALER,
                6 => ROUTER,
                7 => PULL,
                8 => PUSH,
                9 => XPUB,
                10 => XSUB,
                _ => fail!("socket type is out of range!")
            }
        })
    }

    pub fn get_rcvmore(&self) -> Result<bool, Error> {
        getsockopt_i64(self.sock, ZMQ_RCVMORE.to_raw()).and_then (|o| {
            Ok(o == 1i64)
        })
    }

    pub fn get_maxmsgsize(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_MAXMSGSIZE.to_raw())
    }


    pub fn get_sndhwm(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_SNDHWM.to_raw())
    }

    pub fn get_rcvhwm(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_RCVHWM.to_raw())
    }

    pub fn get_affinity(&self) -> Result<u64, Error> {
        getsockopt_u64(self.sock, ZMQ_AFFINITY.to_raw())
    }

    pub fn get_identity(&self) -> Result<Vec<u8>, Error> {
        getsockopt_bytes(self.sock, ZMQ_IDENTITY.to_raw())
    }

    pub fn get_rate(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_RATE.to_raw())
    }

    pub fn get_recovery_ivl(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_RECOVERY_IVL.to_raw())
    }

    pub fn get_recovery_ivl_msec(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_RECOVERY_IVL_MSEC.to_raw())
    }

    pub fn get_mcast_loop(&self) -> Result<bool, Error> {
        getsockopt_i64(self.sock, ZMQ_MCAST_LOOP.to_raw()).and_then(|o| {
            Ok(o == 1i64)
        })
    }

    pub fn get_sndbuf(&self) -> Result<u64, Error> {
        getsockopt_u64(self.sock, ZMQ_SNDBUF.to_raw())
    }

    pub fn get_rcvbuf(&self) -> Result<u64, Error> {
        getsockopt_u64(self.sock, ZMQ_RCVBUF.to_raw())
    }

    pub fn get_linger(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_LINGER.to_raw())
    }

    pub fn get_reconnect_ivl(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_RECONNECT_IVL.to_raw())
    }

    pub fn get_reconnect_ivl_max(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_RECONNECT_IVL_MAX.to_raw())
    }

    pub fn get_backlog(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_BACKLOG.to_raw())
    }

    pub fn get_fd(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_FD.to_raw())
    }

    pub fn get_events(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_EVENTS.to_raw())
    }

    pub fn set_maxmsgsize(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, ZMQ_MAXMSGSIZE.to_raw(), value)
    }

    pub fn set_sndhwm(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_SNDHWM.to_raw(), value)
    }

    pub fn set_rcvhwm(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_RCVHWM.to_raw(), value)
    }

    pub fn set_affinity(&self, value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, ZMQ_AFFINITY.to_raw(), value)
    }

    pub fn set_identity(&self, value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, ZMQ_IDENTITY.to_raw(), value)
    }

    pub fn set_subscribe(&self, value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, ZMQ_SUBSCRIBE.to_raw(), value)
    }

    pub fn set_unsubscribe(&self, value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, ZMQ_UNSUBSCRIBE.to_raw(), value)
    }

    pub fn set_rate(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, ZMQ_RATE.to_raw(), value)
    }

    pub fn set_recovery_ivl(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, ZMQ_RECOVERY_IVL.to_raw(), value)
    }

    pub fn set_recovery_ivl_msec(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, ZMQ_RECOVERY_IVL_MSEC.to_raw(), value)
    }

    pub fn set_mcast_loop(&self, value: bool) -> Result<(), Error> {
        let value = if value { 1i64 } else { 0i64 };
        setsockopt_i64(self.sock, ZMQ_MCAST_LOOP.to_raw(), value)
    }

    pub fn set_sndbuf(&self, value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, ZMQ_SNDBUF.to_raw(), value)
    }

    pub fn set_rcvbuf(&self, value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, ZMQ_RCVBUF.to_raw(), value)
    }

    pub fn set_linger(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_LINGER.to_raw(), value)
    }

    pub fn set_reconnect_ivl(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_RECONNECT_IVL.to_raw(), value)
    }

    pub fn set_reconnect_ivl_max(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_RECONNECT_IVL_MAX.to_raw(), value)
    }

    pub fn set_backlog(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_BACKLOG.to_raw(), value)
    }

    pub fn as_poll_item(&self, events: i16) -> PollItem {
        PollItem {
            socket: self.sock,
            fd: 0,
            events: events,
            revents: 0
        }
    }
}

pub struct Message {
    msg: Msg_
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            let _ = zmq_msg_close(&self.msg);
        }
    }
}

impl Message {
    pub fn new() -> Message {
        unsafe {
            let message = Message { msg: [0, ..32] };
            let _ = zmq_msg_init(&message.msg);
            message
        }
    }

    pub fn with_bytes<T>(&self, f: |&[u8]| -> T) -> T {
        unsafe {
            let data = zmq_msg_data(&self.msg);
            let len = zmq_msg_size(&self.msg) as uint;
            slice::raw::buf_as_slice(data, len, f)
        }
    }

    pub fn with_str<T>(&self, f: |&str| -> T) -> T {
            self.with_bytes(|v| f(str::from_utf8(v).unwrap()))
    }

    pub fn to_bytes(&self) -> ~[u8] {
        self.with_bytes(|v| v.to_owned())
    }

    pub fn to_str(&self) -> String {
        self.with_str(|s| s.to_string())
    }
}

pub static POLLIN : i16 = 1i16;
pub static POLLOUT : i16 = 2i16;
pub static POLLERR : i16 = 4i16;

#[allow(visible_private_types)]
pub struct PollItem {
    socket: Socket_,
    fd: c_int,
    events: i16,
    pub revents: i16
}

pub fn poll(items: &mut [PollItem], timeout: i64) -> Result<(), Error> {
    unsafe {
        let rc = zmq_poll(
            items.as_mut_ptr(),
            items.len() as c_int,
            timeout);
        if rc == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }
}

impl fmt::Show for Error {
    /// Return the error string for an error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "{}",
                   str::raw::from_c_str(zmq_strerror(*self as c_int)))
        }
    }
}

macro_rules! getsockopt_num(
    ($name:ident, $c_ty:ty, $ty:ty) => (
        fn $name(sock: Socket_, opt: c_int) -> Result<$ty, Error> {
            unsafe {
                let mut value: $c_ty = 0;
                let value_ptr = &mut value as *mut $c_ty;
                let mut size = mem::size_of::<$c_ty>() as size_t;

                if -1 == zmq_getsockopt(sock, opt, value_ptr as *mut c_void, &mut size) {
                    Err(errno_to_error())
                } else {
                    Ok(value as $ty)
                }
            }
        }
    )
)

getsockopt_num!(getsockopt_int, c_int, int)
getsockopt_num!(getsockopt_i64, int64_t, i64)
getsockopt_num!(getsockopt_u64, uint64_t, u64)

fn getsockopt_bytes(sock: Socket_, opt: c_int) -> Result<Vec<u8>, Error> {
    unsafe {
        // The only binary option in zeromq is ZMQ_IDENTITY, which can have
        // a max size of 255 bytes.
        let mut size = 255 as size_t;
        let mut value = Vec::with_capacity(size as uint);

        let r = zmq_getsockopt(
            sock,
            opt,
            value.as_mut_ptr() as *mut c_void,
            &mut size);

        if r == -1i32 {
            Err(errno_to_error())
        } else {
            value.truncate(size as uint);
            Ok(value)
        }
    }
}

macro_rules! setsockopt_num(
    ($name:ident, $ty:ty) => (
        fn $name(sock: Socket_, opt: c_int, value: $ty) -> Result<(), Error> {
            unsafe {
                let size = mem::size_of::<$ty>() as size_t;

                if -1 == zmq_setsockopt(sock, opt, (&value as *$ty) as *c_void, size) {
                    Err(errno_to_error())
                } else {
                    Ok(())
                }
            }
        }
    )
)

setsockopt_num!(setsockopt_int, int)
setsockopt_num!(setsockopt_i64, i64)
setsockopt_num!(setsockopt_u64, u64)

fn setsockopt_bytes(sock: Socket_, opt: c_int, value: &[u8]) -> Result<(), Error> {
    unsafe {
        let r = zmq_setsockopt(
            sock,
            opt,
            value.as_ptr() as *c_void,
            value.len() as size_t
        );

        if r == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }
}

fn errno_to_error() -> Error {
    Error::from_raw(unsafe { zmq_errno() })
}
