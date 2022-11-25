#!/bin/bash

token=$(cat token)
guildId=$(cat GuildId)

export DISCORD_TOKEN="$token"
export GUILD_ID="$guildId"
