# Unclosed fence at EOF

CommonMark permits an opening fence with no closing fence before end of file.
Everything after the next line is code, including the apparent heading.

```text
line one
line two
# Not an outline heading
UNCLOSED-FENCE-END
