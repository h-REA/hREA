# HoloREA application DNAs

Each directory within this module corresponds to a separate HoloREA application DNA. You can think of app DNAs as 'lego blocks' for composing more complex distributed systems; like microservices in client / server web applications.

The architecture of HoloREA is designed to be as flexible as possible between components. We aim to separate the overall HoloREA app 'suite' into sensible logical services which allow for composition and pluggability. For example, you might swap an external project management tool for the `planning` DNA; or bring in your own agreement handling functionality.

The broader network architecture is also up to you. Usually we imagine that HoloREA app DNAs will be deployed in groups, but it is also valid, for example, to have two private networks of observed events & resources referencing a shared planning space.
