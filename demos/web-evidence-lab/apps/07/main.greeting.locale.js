// defines: main.greeting.message locale formatting contract
                const words = {en: 'Hello', es: 'Hola'};
                export function format(name, locale = 'en') { return `${words[locale] || words.en}, ${name}!`; }
