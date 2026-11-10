$src = 'C:\Users\27648\AppData\Local\Temp\dsh-spill-kFD5IC\session-145c4c62aff6\aa1c0a9f1d39-web_fetch.txt'
$raw = Get-Content -Raw -LiteralPath $src
$feed = [System.Net.WebUtility]::HtmlDecode($raw)
$out = New-Object System.Collections.Generic.List[string]
$em = [regex]::Matches($feed, '(?m)^    <id>tag:github\.com,2008:Repository/299354207/(?<t>[^<]+)</id>\s*\n\s*<updated>(?<d>[^<]+)</updated>')
foreach ($m in $em) {
  $tag = $m.Groups['t'].Value
  if ($tag -notin @('1.4.6','1.4.5','1.4.4','1.4.3')) { continue }
  $date = $m.Groups['d'].Value
  $start = $m.Index
  $nextStart = $feed.Length
  foreach ($m2 in $em) { if ($m2.Index -gt $start) { $nextStart = $m2.Index; break } }
  $block = $feed.Substring($start, $nextStart - $start)
  $ci = $block.IndexOf('<details>')
  if ($ci -lt 0) { continue }
  $details = $block.Substring($ci)
  $de = $details.IndexOf('</details>')
  if ($de -ge 0) { $details = $details.Substring(0, $de) }
  $out.Add("##### $tag  updated=$date")
  $sections = [regex]::Matches($details, '<h1>(?<n>Added|Changes|Fixes)</h1>')
  for ($i = 0; $i -lt $sections.Count; $i++) {
    $s = $sections[$i]
    $segEnd = if ($i -lt $sections.Count - 1) { $sections[$i + 1].Index } else { $details.Length }
    $seg = $details.Substring($s.Index + $s.Length, $segEnd - $s.Index - $s.Length)
    $out.Add('== ' + $s.Groups['n'].Value)
    foreach ($lm in [regex]::Matches($seg, '<li>(?<body>[\s\S]*?)</li>')) {
      $body = $lm.Groups['body'].Value
      $nums = New-Object System.Collections.Generic.List[string]
      foreach ($am in [regex]::Matches($body, '<a [^>]*href="(?<u>https://github\.com/rustdesk/rustdesk/(?:pull|issues|discussions|commit)/[^"]+)"')) {
        $u = $am.Groups['u'].Value
        if ($u -match '/(pull|issues)/(\d+)$') { $nums.Add('#' + $matches[2]) }
        elseif ($u -match '/discussions/(\d+)') { $nums.Add('#' + $matches[1]) }
        elseif ($u -match '/commit/([0-9a-f]{7,})') { $nums.Add($matches[1].Substring(0, 8)) }
      }
      $text = [regex]::Replace($body, '<[^>]+>', '')
      $text = [regex]::Replace($text, '\s+', ' ').Trim()
      if ($nums.Count -gt 0) {
        $joined = ($nums | Select-Object -Unique) -join ' '
        if ($text -notmatch [regex]::Escape($joined.Split(' ')[0])) { $text = ($text + ' ' + $joined).Trim() }
      }
      $out.Add('- ' + $text)
    }
  }
  $out.Add('')
}
$dst = Join-Path (Get-Location) 'rustdesk_1.4.3-1.4.6_changelog_extract.txt'
Set-Content -LiteralPath $dst -Value $out -Encoding UTF8
"lines: $($out.Count)"; "file: $dst"
