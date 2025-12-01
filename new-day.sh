#!/usr/bin/env -S bash

if ! [[ ( $1 =~ ^[0-9]{1,2}$ ) && ( $2 ) ]] ; then
  echo 'Usage: new-day.sh {day} {name}'
  exit 1
fi

folder="$(printf '%02d' $1)-${2}"

cp -r "./00-template" "./$folder"

sed -i "s/template/$2/" "./$folder/Cargo.toml" "./$folder/src/bin/part1.rs" "./$folder/src/bin/part2.rs"
