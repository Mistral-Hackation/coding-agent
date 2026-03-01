use crate::agentic::errors::AgentError;

/// Parses a delegation response from an agent.
///
/// Expected format: `DELEGATE: <agent_name> <instruction>`
/// Returns a tuple of `(agent_name, instruction)`.
pub fn parse_delegation(response: &str) -> Result<(String, String), AgentError> {
    // Expected format: "DELEGATE: <agent> <instruction>"
    let part = response
        .split("DELEGATE:")
        .nth(1)
        .ok_or_else(|| AgentError::DelegationParseError("Missing DELEGATE keyword".into()))?;

    let part = part.trim();
    if let Some((target, instruction)) = part.split_once(' ') {
        // Sanitize agent name: keep only alphanumeric and underscore, take first line
        let clean_target: String = target
            .trim()
            .lines()
            .next()
            .unwrap_or(target)
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>()
            .to_lowercase();

        // Validate it's not empty
        if clean_target.is_empty() {
            return Err(AgentError::DelegationParseError(format!(
                "Invalid agent name parsed: '{}'",
                target
            )));
        }

        Ok((clean_target, instruction.to_string()))
    } else {
        // If no instruction, assume empty - but still sanitize target
        let clean_target: String = part
            .trim()
            .lines()
            .next()
            .unwrap_or(part)
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>()
            .to_lowercase();

        if clean_target.is_empty() {
            return Err(AgentError::DelegationParseError(format!(
                "Invalid agent name parsed: '{}'",
                part
            )));
        }

        Ok((clean_target, "".to_string()))
    }
}

/// Parses a tool call response from an agent.
///
/// Expected format: `TOOL: <tool_name> <args>`
/// Returns a tuple of `(tool_name, args)`.
#[allow(dead_code)]
pub fn parse_tool(response: &str) -> Result<(String, String), AgentError> {
    // Expected format: "TOOL: <name> <args>"
    let part = response
        .split("TOOL:")
        .nth(1)
        .ok_or_else(|| AgentError::DelegationParseError("Missing TOOL keyword".into()))?;

    let part = part.trim();
    if let Some((name, args)) = part.split_once(' ') {
        Ok((name.to_string(), args.trim().to_string()))
    } else {
        // Assume just tool name, no args (unlikely but safe)
        Ok((part.to_string(), "".to_string()))
    }
}
