// Page setup
#set page(paper: "a4", margin: 2cm)
#set text(font: "Noto Sans", size: 10pt)
#set heading(numbering: none)

// --- Header Section ---
#align(center)[
  #text(size: 24pt, weight: "bold", fill: rgb("#111827"))[MadTrack Expense Report]\
  #v(0.5cm)
  #text(size: 16pt, weight: "medium", fill: rgb("#6b7280"))[February 2026]
]

#v(1.5cm)

// --- Summary Cards Section ---
#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 1cm,
  
  // Income Card
  block(
    fill: rgb("#f0fdf4"), inset: 15pt, radius: 8pt, width: 100%,
    align(center)[
      #text(size: 11pt, fill: rgb("#166534"), weight: "bold")[Total Income]\
      #v(8pt)
      #text(size: 16pt, weight: "bold", fill: rgb("#15803d"))[#calc.round(data.total_income, digits: 2) MAD]
    ]
  ),
  
  // Expenses Card
  block(
    fill: rgb("#fef2f2"), inset: 15pt, radius: 8pt, width: 100%,
    align(center)[
      #text(size: 11pt, fill: rgb("#991b1b"), weight: "bold")[Total Expenses]\
      #v(8pt)
      #text(size: 16pt, weight: "bold", fill: rgb("#b91c1c"))[#calc.round(data.total_expenses, digits: 2) MAD]
    ]
  ),
  
  // Balance Card (Dynamic Colors)
  block(
    fill: if data.balance >= 0 { rgb("#f0fdf4") } else { rgb("#fef2f2") },
    inset: 15pt, radius: 8pt, width: 100%,
    align(center)[
      #text(size: 11pt, fill: if data.balance >= 0 { rgb("#166534") } else { rgb("#991b1b") }, weight: "bold")[Balance]\
      #v(8pt)
      #text(size: 16pt, weight: "bold", fill: if data.balance >= 0 { rgb("#15803d") } else { rgb("#b91c1c") })[#calc.round(data.balance, digits: 2) MAD]
    ]
  )
)

#v(1.5cm)

// --- Transactions Table Section ---
#text(size: 14pt, weight: "bold", fill: rgb("#111827"))[Recent Transactions]
#v(0.5cm)

#table(
  columns: (auto, 1fr, auto, auto),
  stroke: none,
  // Alternating row colors, with a distinct header color
  fill: (_, row) => if row == 0 { rgb("#f3f4f6") } else if calc.rem(row, 2) == 0 { rgb("#f9fafb") } else { white },
  // Right-align the Amount column (index 2)
  align: (col, _) => if col == 2 { right } else { left },
  
  table.header(
    [*ID*], [*Category*], [*Amount*], [*Date*]
  ),
  
  ..data.transactions.map(t =>
    (
      [#t.id],
      [#t.category],
      [#calc.round(t.amount, digits: 2) MAD],
      [#t.date],
    )
  ).flatten()
)

#v(1.5cm)

// --- Top Categories Section ---
#text(size: 14pt, weight: "bold", fill: rgb("#111827"))[Top Categories]
#v(0.5cm)

#grid(
  columns: (1fr, 1fr),
  gutter: 15pt,
  ..data.category_breakdown.map(cat =>
    block(
      fill: rgb("#f9fafb"),
      inset: 12pt,
      radius: 6pt,
      width: 100%,
      stroke: 1pt + rgb("#e5e7eb"),
      [
        #grid(
          columns: (1fr, auto),
          [ *#cat.category* ],
          [ #text(weight: "bold")[#calc.round(cat.amount, digits: 2) MAD] ]
        )
        #v(5pt)
        #text(size: 9pt, fill: rgb("#6b7280"))[#calc.round(cat.percentage, digits: 1)% of total expenses]
      ]
    )
  )
)

// Push footer to the absolute bottom of the page
#v(1fr) 

// --- Footer Section ---
#line(length: 100%, stroke: 0.5pt + rgb("#d1d5db"))
#v(5pt)
#align(
  right,
  text(
    size: 9pt,
    fill: rgb("#6b7280"),
    [Generated on #datetime.today().display() | Total transactions: #data.transaction_count]
  )
)
