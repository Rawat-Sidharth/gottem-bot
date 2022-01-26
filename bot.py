import discord
import os
from dotenv import load_dotenv

load_dotenv()
client = discord.Client()

@client.event
async def on_ready():
    print('{0.user} online!'.format(client))

@client.event
async def on_message(message):
    if message.author == client.user:
        return
    else:
        content = message.content
        if content.startsWith('!ligma'):
            await message.channel.send('ligma balls! GOTTEM')
        elif content.startsWith('!sugma'):
            await message.channel.send('sugma dick! GOTTEM')


client.run(os.getenv('TOKEN'))
