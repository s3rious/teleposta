import { Bot } from "grammy";
import { type Args, parseArgs } from "parseArgs";

type LaunchArgs = Args<{
  token: string;
  channel: string;
  message: string;
  template?: string;
  verbose?: boolean;
}>;

async function startBot(token: string): Promise<Error | Bot> {
  try {
    const bot = new Bot(token);

    return await new Promise((resolve, reject) => {
      bot
        .start({
          drop_pending_updates: true,
          onStart() {
            resolve(bot);
          },
        })
        .catch(reject);
    });
  } catch (error) {
    return error;
  }
}

function getFormattedMessage(message: string, template?: string): string {
  if (!template) {
    return message;
  }

  const messageObject = JSON.parse(message);
  const keys = Object.keys(messageObject);
  const values = Object.values(messageObject);

  return new Function(...keys, `return \`${template}\`;`)(...values);
}

function sendMessageTo(bot: Bot, channel: string, message: string) {
  return bot.api.sendMessage(channel, message, {
    parse_mode: "HTML",
  });
}

async function main() {
  const args = parseArgs<LaunchArgs>(Deno.args);
  const { token, channel, template, message, verbose } = args;

  if (!token) {
    throw new Error(
      "There no telegram token provided via `--token` argument, can't work without it",
    );
  }

  if (!channel) {
    throw new Error(
      "There is no telegram channel provided via --channel, can't post a message into nowhere",
    );
  }

  if (!message) {
    throw new Error(
      "There is no message provided via --message, can't post nothing",
    );
  }

  if (verbose) {
    console.log("Staring bot...");
  }

  const bot = await startBot(token);

  if (bot instanceof Error) {
    return console.error(bot);
  }

  if (verbose) {
    console.log("... started!..");
  }

  try {
    if (verbose) {
      console.log("... formatting message to post...", message, template);
    }

    const formattedMessage = getFormattedMessage(message, template);

    if (verbose) {
      console.log("... formatted!..", formattedMessage);
    }

    if (verbose) {
      console.log(
        "... posting a message into channel...",
        channel,
        formattedMessage,
      );
    }

    const result = await sendMessageTo(bot, channel, formattedMessage);

    if (verbose) {
      console.log("... posted!..");
    }

    console.log(result);

    if (verbose) {
      console.log("... shutting down bot...");
    }

    await bot.stop();

    if (verbose) {
      console.log("... down!");
    }
  } catch (error) {
    console.error(error);

    bot.stop();
  }
}

await main();
