/// Almost all of these functions are designed to fail silently, 
/// as the programs they're written for only have file access as an afterthought.
/// If file IO is needed for anything where silent failure is not an option, it will need to be reworked.
pub mod file;
