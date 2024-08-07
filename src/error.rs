use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
/// File system errors.
pub enum OkuFsError {
    #[error("File system entry not found.")]
    #[diagnostic(
        code(fs::fs_entry_not_found),
        url(docsrs),
        help("Please ensure that the file system entry exists before attempting to read it.")
    )]
    /// File system entry not found.
    FsEntryNotFound,
    #[error("Author cannot be created.")]
    #[diagnostic(code(fs::author_cannot_be_created), url(docsrs))]
    /// Author cannot be created.
    AuthorCannotBeCreated,
    #[error("Cannot start node.")]
    #[diagnostic(code(fs::cannot_start_node), url(docsrs))]
    /// Cannot start node.
    CannotStartNode,
    #[error("Cannot retrieve list of authors.")]
    #[diagnostic(code(fs::cannot_retrieve_authors), url(docsrs))]
    /// Cannot retrieve list of authors.
    CannotRetrieveAuthors,
    #[error("Cannot retrieve node address.")]
    #[diagnostic(code(fs::cannot_retrieve_node_address), url(docsrs))]
    /// Cannot retrieve node address.
    CannotRetrieveNodeAddress,
    #[error("Cannot stop node.")]
    #[diagnostic(code(fs::cannot_stop_node), url(docsrs))]
    /// Cannot stop node.
    CannotStopNode,
    #[error("Cannot create replica.")]
    #[diagnostic(code(fs::cannot_create_replica), url(docsrs))]
    /// Cannot create replica.
    CannotCreateReplica,
    #[error("Cannot exit replica.")]
    #[diagnostic(code(fs::cannot_exit_replica), url(docsrs))]
    /// Cannot exit replica.
    CannotExitReplica,
    #[error("Cannot delete replica.")]
    #[diagnostic(code(fs::cannot_delete_replica), url(docsrs))]
    /// Cannot delete replica.
    CannotDeleteReplica,
    #[error("Cannot list replicas.")]
    #[diagnostic(code(fs::cannot_list_replicas), url(docsrs))]
    /// Cannot list replicas.
    CannotListReplicas,
    #[error("Cannot open replica.")]
    #[diagnostic(code(fs::cannot_open_replica), url(docsrs))]
    /// Cannot open replica.
    CannotOpenReplica,
    #[error("Cannot list files.")]
    #[diagnostic(code(fs::cannot_list_files), url(docsrs))]
    /// Cannot list files.
    CannotListFiles,
    #[error("Cannot create or modify file.")]
    #[diagnostic(code(fs::cannot_create_or_modify_file), url(docsrs))]
    /// Cannot create or modify file.
    CannotCreateOrModifyFile,
    #[error("Cannot delete file.")]
    #[diagnostic(code(fs::cannot_delete_file), url(docsrs))]
    /// Cannot delete file.
    CannotDeleteFile,
    #[error("Cannot read file.")]
    #[diagnostic(code(fs::cannot_read_file), url(docsrs))]
    /// Cannot read file.
    CannotReadFile,
    #[error("Cannot delete directory.")]
    #[diagnostic(code(fs::cannot_delete_directory), url(docsrs))]
    /// Cannot delete directory.
    CannotDeleteDirectory,
}

#[derive(Error, Debug, Diagnostic)]
/// Content discovery errors.
pub enum OkuDiscoveryError {
    #[error("Invalid hash and format.")]
    #[diagnostic(code(discovery::invalid_hash_and_format), url(docsrs))]
    /// Invalid hash and format.
    InvalidHashAndFormat,
    #[error("Problem announcing {0} ({1}).")]
    #[diagnostic(code(discovery::problem_announcing_content), url(docsrs))]
    /// Problem announcing content.
    ProblemAnnouncingContent(String, String),
    #[error("Cannot generate sharing ticket for replica.")]
    #[diagnostic(code(discovery::cannot_generate_sharing_ticket), url(docsrs))]
    /// Cannot generate sharing ticket for replica.
    CannotGenerateSharingTicket,
    #[error("Cannot generate sharing ticket for file(s).")]
    #[diagnostic(code(discovery::cannot_generate_sharing_ticket_for_files), url(docsrs))]
    /// Cannot generate sharing ticket for file(s).
    CannotGenerateSharingTicketForFiles,
}

#[derive(Error, Debug, Diagnostic)]
/// Relay errors.
pub enum OkuRelayError {
    #[error("No connected node can satisfy {0}.")]
    #[diagnostic(code(relay::cannot_satisfy_request), url(docsrs))]
    /// No connected node can satisfy request.
    CannotSatisfyRequest(String),
    #[error("Problem connecting to {0}.")]
    #[diagnostic(code(relay::problem_connecting), url(docsrs))]
    /// Problem connecting to node.
    ProblemConnecting(String),
}
