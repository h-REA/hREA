export function capitalize(string) {
    if (!string) return '';
    return string.charAt(0).toUpperCase() + string.slice(1);
}

export function pluralize(string) {
    if (!string) return '';
    if (string.endsWith('s')) {
        return string + 'es';
    } else if (string.endsWith('y')) {
        return string.slice(0, -1) + 'ies';
    } else {
        return string + 's';
    }
};

export function parseGraphQLFields(input) {
  const lines = input.split('\n');

  const output = {};

  for (const line of lines) {
    // Remove docstring lines
    if (line.trim().startsWith('"""') || line.trim().startsWith('"')) continue;

    const trimmed = line.trim();
    if (!trimmed) continue;

    // Match GraphQL field line: name(args): ReturnType
    const match = trimmed.match(/^(\w+)\s*(\([^\)]*\))?\s*:\s*(.+)$/);
    if (match) {
      const [, name, args, returnType] = match;
      output[name] = {
        returnType: returnType.trim(),
        args: args ? args.trim() : null
      };
    }
  }

  return output;
}
