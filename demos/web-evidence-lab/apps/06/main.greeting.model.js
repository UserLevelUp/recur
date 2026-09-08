// defines: main.greeting.name normalized input contract
                export function normalize(value) {
                  const name = value.trim();
                  if (!name) throw new Error('Please enter a name.');
                  if (name.length > 40) throw new Error('Use 40 characters or fewer.');
                  return name;
                }
