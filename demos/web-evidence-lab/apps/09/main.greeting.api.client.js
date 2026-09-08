// consumes: main.greeting.api.response server greeting contract
                export async function request(name, locale) {
                  const response = await fetch(`/api/greeting?${new URLSearchParams({name, locale})}`);
                  if (!response.ok) throw new Error(`Greeting service unavailable (${response.status}).`);
                  return (await response.json()).message;
                }
