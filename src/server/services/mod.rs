use self::architect_services::DynArchitectService;

pub mod architect_services;


// TODO: I think i'm supposed to bring my 'architect' repository here.... that doesn't make sense?
pub struct Services {
    pub architects: DynArchitectService
}
