//! WebSocket API only! The path_find method searches for a path along which a
//! transaction can possibly be made, and periodically sends updates when the
//! path changes over time. For a simpler version that is supported by
//! JSON-RPC, see the ripple_path_find method. For payments occurring strictly
//! in XRP, it is not necessary to find a path, because XRP can be sent
//! directly to any account.
//!
//! There are three different modes, or sub-commands, of the path_find command.
//! Specify which one you want with the subcommand parameter:
//!
//! create - Start sending pathfinding information
//! close - Stop sending pathfinding information
//! status - Get the information of the currently-open pathfinding request
//!
//! <https://xrpl.org/path_find.html>

// #TODO
