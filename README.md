# teleposta

Usage
deno run --allow-net=api.telegram.org main.ts --token=<TOKEN> --channel=<CHANNEL> --message="{\"title\":\"title\",\"body\":\"body\"}" --template="<b>\${title}</b>\n\n\${body}"