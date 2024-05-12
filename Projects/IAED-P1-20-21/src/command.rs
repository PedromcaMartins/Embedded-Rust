use crate::{ParserErrorType, VerifierErrorType, AppError};

mod t_command;
mod l_command;
mod n_command;
mod u_command;
mod m_command;
mod d_command;
mod a_command;

pub use t_command::TCommand;
pub use l_command::LCommand;
pub use n_command::NCommand;
pub use u_command::UCommand;
pub use m_command::MCommand;
pub use d_command::DCommand;
pub use a_command::ACommand;

pub trait Command {
    fn parse_and_verify_arguments_and_execute<T: Command>(args: &str) -> Result<(), AppError>
    where
        T: Sized + Default
    {
        let command: T = Command::parse_arguments(args)?;

        command.verify_parameters()?;

        command.execute();

        Ok(())
    }

    fn parse_arguments(args: &str) -> Result<Self, ParserErrorType>
    where
        Self: Sized + Default;

    fn verify_parameters(&self) -> Result<(), VerifierErrorType>;

    fn execute(self);
}