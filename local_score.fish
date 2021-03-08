set score 0
for val in (seq 0 9)
  echo $val
  cargo run --bin ahc001-a < tools/in/000$val.txt ^tmp --release
  cat tmp | while read line
    set now $line
  end
  set score (math $score + $now)
end

for val in (seq 10 49)
  echo $val
  cargo run --bin ahc001-a < tools/in/00$val.txt ^tmp --release
  cat tmp | while read line
    set now $line
  end
  set score (math $score + $now)
end
echo $score