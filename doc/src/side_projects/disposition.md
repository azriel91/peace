# 📐 disposition

Diagrams to SVG.

* Source: <https://github.com/azriel91/disposition>

Decisions:

* **Diagrams:** Represents things or a process.
* **SVG:** Portable, supports styling, interactivity, can be generated in-memory -- browser not needed.

This is a new library / app intended to take the place of [`dot_ix`][`dot_ix`].


## Background

[`dot_ix`][`dot_ix`] is a useful tool to generate diagrams backing on to GraphViz and Tailwind CSS.

The following learnings have come from using [`dot_ix`][`dot_ix`] for 2 years:

1. Requiring GraphViz to be installed / a browser to run the WASM version limits the ability to write a headless application and how cleanly we can write a web UI.
2. GraphViz's layout engine is not predictable -- positioning of nodes is unstable when edges are added -- and this requires the input to be tuned to get an "expected" positioning of nodes / edges.
3. The `dot_ix` input structure is relatively good, but can be further refined with better top-level concepts (`node_type`, better tag support).
4. Native markdown support is desired.

### Alternatives

<details>

* [`dot_ix`][`dot_ix`]: This is/was the previous project, so we're creating the next evolution of it.
* **Browser web driver:** We generate HTML, browser renders it, and we generate SVG off the browser DOM element positions.

    Doesn't work great for CLI -- need to have a headless browser.

</details>


## Design

There are multiple parts to generating a diagram:

1. **Diagram Structure:** Capturing the information that the diagram represents in a suitable data structure.
2. **Layout Document Object Model (DOM):** Calculating the DOM element structure (visual hierarchy) of that information.
3. **Layout:** Calculating the positions of the layout DOM on a viewport with fixed dimensions.
4. **Full Document Object Model (DOM):** Adding edges after the elements are positioned, and adding the attributes that the layout DOM doesn't have.
5. **Rendering:** Producing the visual representation from the full DOM elements.


### 1. Diagram Structure / Capturing Information

Capturing the information for the diagram, in a structure that is easy to reason about and work with. Ideally easy for both humans and computers to read and write.

* **Input:** Input formats, e.g. JSON, YAML, in-memory objects, etc.
* **Output:** Diagram data structure.


#### 1.1. Nodes / Clusters

1. Stable IDs
2. Display names
3. Descriptions / additional detail
4. "type"(s) -- the primary way how this node / cluster should be rendered
5. Tags -- what groups it is part of / affected by.
6. Hierarchy / Nesting / Level of detail

Kinds of diagrams we want to support:

1. **Things:** Shows things, where they are, and their relationship with other things.
2. **Process:** Shows steps in a process, and status / progress of each step.

We don't want to use the names "entity diagram" or "sequence diagram", because it can cause confusion with those terms in the software development context.


#### 1.2. Edges

1. From / to which node / cluster.
2. Direction
3. Type
4. Multiple edges between nodes
5. Edges on the correct point (north, south, east, west) on the node.


### 2. Document Object Model (DOM)

Turn the diagram data structure into DOM elements.

* **Input:** Diagram data structure.
* **Output:** Layout DOM elements which are not viewport bound.

We need to choose one or a combination of:

* HTML DOM
* [`taffy`][`taffy`] format.

ℹ️ **Note:** we also need to consider edge descriptions -- how do we place these in the DOM? `taffy` will be used for flex / grid layout, but where would we place edge DOM elements?


### 3. Layout

Placement of nodes, padding, reflowing text, etc.

* **Input:** Layout DOM elements which are not viewport bound.
* **Output:** DOM elements / text with XY coordinates in a fixed viewport, with tailwind classes.

If we use:

* [`taffy`][`taffy`]: We need to translate its output into the SVG DOM.
* [`blitz`][`blitz`]: We need to translate HTML elements into SVG DOM.
* Headless browser: We need to translate HTML elements into SVG DOM, and we'd need a headless browser, which isn't convenient for CI.

ℹ️ **Note:** we also need to consider edge descriptions -- if there is a lot of text, should we have spacing for those labels?


#### 3.1. DOM representation

Because we want node descriptions to be markdown, we need to convert them to an appropriate DOM structure that can represent the rendered markdown, as well as encode the layout and styling information.


##### 3.1.1. Option 1: SVG

1. We have to calculate the positions of nodes and text ourselves, including padding etc.
2. Markdown is converted to HTML, then we use those to position the text.
3. i.e. we'd have to know / calculate the font metrics for bold/italicized text.
4. [`taffy`][`taffy`] is what `servo` uses, and does element layouting.
5. [`cosmic-text`](https://crates.io/crates/cosmic-text) is needed for text width calculations.
6. See the [`cosmic_text` example](https://github.com/DioxusLabs/taffy/blob/v0.9.1/examples/cosmic_text/src/main.rs) -- you need font metrics to know how text renders.


##### 3.1.2. Option 2: HTML + HTML to SVG

1. HTML rendering engine does the layout of text positioning for us.
2. Markdown will easily be supported here, because we can convert to HTML, then the rendering engine takes care of the rest.


#### 3.2. Images

Images can be inlined in markdown, and based on the image data or a provided value, we can pass that to `taffy` to calculate the position. If we use [`comrak`][`comrak`], then we need to wait for [`comrak#586`][`comrak#586`] to be resolved to get the passed in dimensions of the image.


### 4. Full Document Object Model (DOM)

Adding edges after the elements are positioned, and adding the attributes that the layout DOM doesn't have.

* **Input:** Layout DOM elements with fixed coordinates.
* **Output:** Render DOM elements (including text) with XY coordinates in a fixed viewport, with tailwind classes.


### 5. Rendering

Rendering of the DOM into a visual and interactive format.

* **Input:** Render DOM elements with XY coordinates in a fixed viewport, with tailwind classes.
* **Output:** Visual and interactive diagram.

Any browser could render HTML / SVG. If we want a non-browser solution, look at:

1. [`stylo`](https://crates.io/crates/stylo) is the CSS engine servo uses. Do we need it? If we do, there's [`stylo_taffy`](https://crates.io/crates/stylo_taffy)
2. [`blitz`][`blitz`] seems to be doing what we want (and more), but in HTML.
3. [`blitz`#260](https://github.com/DioxusLabs/blitz/issues/260) is where they'd add SVG support.


### Solution

Probably:

1. Define diagram structure based on `dot_ix`'s learnings.
2. Map the structure to [`taffy`][`taffy`]'s elements.
3. Use [`taffy`][`taffy`] to lay out the diagram.
4. Convert to SVG, adding edges and attributes from the input structure. [`kurbo`][`kurbo`] may be useful to compute the edge path coordinates.
5. Return that to the caller -- SVG can be rendered in a browser. In the future, we might use [`blitz`][`blitz`] to render the SVG.


## Ideas / Learnings from `dot_ix`

1. Ability to combine both `Thing` diagrams and `Process` diagrams. i.e. a process diagram whose steps show what is happening on the things. Maybe we just have one kind of diagram that does both.

    1. Maybe we just have one kind of diagram that does both.
    2. What `dot_ix` has as tags can be distinguished as `Process`es or `TagGroup`s.

2. When a node is styled with certain colours, apply it to all child nodes.
3. Light / Medium / Dark presets for shading.
4. Dependency diagrams: is it possible to select a node, and a menu appears, with buttons each to highlight:

    1. All transitive dependencies this depends on.
    2. All transitive dependents that depend on this.
    3. Immediate neighbours.

    Need to experiment with `group-focus-within` from tailwind.

    Looks like it may be possible with different CSS pseudo classes / pseudo elements.

    e.g. when an element is clicked on, it becomes the `:target` element in the document, and the css selector `#element-id:target ~ #other` allows you to style `#other` when `#element-id` was clicked, presumably when the focus is changed from  `#element-id` to something else.

[`blitz`]: https://github.com/DioxusLabs/blitz
[`comrak`]: https://github.com/kivikakk/comrak
[`comrak#586`]: https://github.com/kivikakk/comrak/issues/586
[`dot_ix`]: https://azriel.im/dot_ix/
[`kurbo`]: https://github.com/linebender/kurbo
[`taffy`]: https://github.com/DioxusLabs/taffy


## Example Input

````yaml
---
things: &things
  aws: "☁️ Amazon Web Services"
  aws_iam: "🖊️ Identity and Access Management"
  aws_iam_ecs_policy: "🗒️ ECS IAM Policy"
  aws_ecr: "🗄️ Elastic Container Registry"
  aws_ecr_repo: "💽 web_app repo"
  aws_ecr_repo_image_1: "💿 Image 1"
  aws_ecr_repo_image_2: "💿 Image 2"
  aws_ecs: "💻 Elastic Container Service"
  aws_ecs_cluster_app: "🎛️ web_app cluster"
  aws_ecs_cluster_app_task: "🖨️ web_app task version 1"
  github: "🐙 GitHub"
  github_user_repo: "azriel91/web_app"
  localhost: "🧑‍💻 Localhost"
  localhost_repo: "📂 ~/work/web_app"
  localhost_repo_src: "📝 src"
  localhost_repo_target: "📂 target"
  localhost_repo_target_file_zip: "📝 file.zip"
  localhost_repo_target_dist_dir: "📁 dist"

# Render a copy text button, and, when clicked,
# what text to place on the clipboard.
thing_copy_text:
  <<: *things
  localhost_repo: "~/work/web_app"
  localhost_repo_src: "~/work/web_app/src"
  localhost_repo_target: "~/work/web_app/target"
  localhost_repo_target_file_zip: "~/work/web_app/target/file.zip"
  localhost_repo_target_dist_dir: "~/work/web_app/target/dist"

# This defines the nesting, but perhaps we should use it to define the relative positioning as well.
#
# Do we want users to have control, e.g. placing the things in a grid?
#
# Other question, the positioning for a software dependency tree is different to the positioning
# for deployment topology. Maybe we allow the user to specify either "rank" based layout or "flow"
# based layout.
thing_hierarchy:
  aws:
    aws_iam:
      aws_iam_ecs_policy: {}
    aws_ecr:
      aws_ecr_repo:
        aws_ecr_repo_image_1: {}
        aws_ecr_repo_image_2: {}

  github:
    github_user_repo: {}

  localhost:
    localhost_repo:
      localhost_repo_src: {}
      localhost_repo_target:
        localhost_repo_target_file_zip: {}
        localhost_repo_target_dist_dir: {}

# Dependencies between things can be one way, or cyclic.
#
# Dependencies are static relationships between things, and should be rendered as "on" or "off"
# depending on whether a `thing` is focused / targeted, and whether the user wants to see:
#
# * Predecessors / successors linked to this thing.
# * Immediate dependencies vs transitive (maybe closest `n` neighbours).
#
# * When B depends on A, it means A must exist before B.
# * Changes to A means B is out of date.
#
# How we render dependencies (forward / backward / undirected / bidirectional arrows) can be
# defined separately from the meaning of the dependency.
#
# We want to make it easy to define dependencies between chains of things.
thing_dependencies: &thing_dependencies
  edge_localhost__github_user_repo__pull:
    # Last thing in the list has an edge back to first thing.
    #
    # Should have at least one `thing`.
    cyclic:
      - localhost
      - github_user_repo
  edge_localhost__github_user_repo__push:
    # 2 or more things
    sequence:
      - localhost
      - github_user_repo
  edge_localhost__localhost__within:
    cyclic:
      - localhost
  edge_github_user_repo__github_user_repo__within:
    cyclic:
      - github_user_repo
  edge_github_user_repo__aws_ecr_repo__push:
    sequence:
      - github_user_repo
      - aws_ecr_repo
  edge_aws_ecr_repo__aws_ecs_service__push:
    sequence:
      - aws_ecr_repo
      - aws_ecs_service

thing_dependencies_descs:
  edge_localhost__github_user_repo__pull: ~
  edge_localhost__github_user_repo__push: ~
  edge_localhost__localhost__within: ~
  edge_github_user_repo__github_user_repo__within: ~
  edge_github_user_repo__aws_ecr_repo__push: ~
  edge_aws_ecr_repo__aws_ecs_service__push: ~

# Interactions between things can be one way, or cyclic.
#
# Interactions have the same data structure as dependencies, but are conceptually different:
# `thing_dependencies` is intended to represent dependencies between software libraries,
# while interactions are communication between applications.
# 
# There *are* ordering dependencies between interactions, but *when* it is useful to render
# `thing_dependencies` and `thing_interactions` differ. Dependencies are static at a point in time,
# so it is useful to render the links between multiple `thing`s; interactions are present when a
# step in a process is executing, so they are rendered when the step is focused.
#
# IDs here can be the same as the ones in `thing_dependencies`.
#
# We want to make it easy to define interactions between chains of things.
thing_interactions: *thing_dependencies # cheat and use yaml reference

processes:
  proc_app_dev:
    name: "App Development"
    desc: |-
      Development of the web application.

      * [🐙 Repo](https://github.com/azriel91/web_app)
    steps:
      proc_app_dev_step_repository_clone: "Clone repository"
      proc_app_dev_step_project_build: "Build project"
    step_descs:
      proc_app_dev_step_repository_clone: |-
        ```bash
        git clone https://github.com/azriel91/web_app.git
        ```

      proc_app_dev_step_project_build: |-
        Develop the app:

        * Always link to issue.
        * Open PR.

    # Thing interactions that should be actively highlighted when this step is focused.
    #
    # The things associated with all of the interaction IDs in the list should be highlighted.
    #
    # optional, references IDs in `thing_interactions` top level element.
    step_thing_interactions:
      proc_app_dev_step_repository_clone: [edge_localhost__github_user_repo__pull]
      proc_app_dev_step_project_build: [edge_localhost__localhost__within]

  proc_app_release:
    steps:
      proc_app_release_step_crate_version_update: "Update crate versions"
      proc_app_release_step_pull_request_open: "Open PR"
      proc_app_release_step_tag_and_push: "Tag and push"
      proc_app_release_step_gh_actions_build: "Github Actions build"
      proc_app_release_step_gh_actions_publish: "Github Actions publish"

    step_descs:
      proc_app_release_step_crate_version_update: |-
        ```bash
        sd -s 'version = "0.3.0"' 'version = "0.3.0"' $(fd -tf -F toml) README.md src/lib.rs
        ```
      proc_app_release_step_pull_request_open: |-
        Create a pull request as usual.
      proc_app_release_step_gh_actions_build: |-
        Github Actions will build the image.
      proc_app_release_step_tag_and_push: |-
        When the PR is merged, tag the commit and push the tag to GitHub.

        ```bash
        git tag 0.3.0
        git push origin 0.3.0
        ```

        The build will push the new version to ECR automatically.
      proc_app_release_step_gh_actions_publish: |-
          Github Actions will publish the image to AWS ECR.

    step_thing_interactions:
      proc_app_release_step_crate_version_update: [edge_localhost__localhost__within]
      proc_app_release_step_pull_request_open: [edge_localhost__github_user_repo__pull]
      proc_app_release_step_tag_and_push: [edge_localhost__github_user_repo__push]
      proc_app_release_step_gh_actions_build: [edge_github_user_repo__github_user_repo__within]
      proc_app_release_step_gh_actions_publish: [edge_github_user_repo__aws_ecr_repo__push]

  # Some processes not defined yet.
  #
  # proc_i12e_global_deploy: {}
  # proc_i12e_region_mgmt_deploy: {}
  # proc_i12e_region_tier_subnets_deploy: {}
  proc_i12e_region_tier_app_deploy:
    steps:
      proc_i12e_region_tier_app_deploy_step_ecs_service_update: "Update ECS service"

    step_descs:
      proc_i12e_region_tier_app_deploy_step_ecs_service_update: |-
        Deploy or update the existing ECS service with the new image.
    step_thing_interactions:
      proc_i12e_region_tier_app_deploy_step_ecs_service_update: [edge_aws_ecr_repo__aws_ecs_service__push]

# Tags are labels that can be associated with things, so that the things can be highlighted when
# the tag is focused.
tags:
  tag_app_development: "Application Development"
  tag_deployment: "Deployment"

# Things and edges that are associated with each tag.
# 
# It probably makes sense to specify the `things` for each tag, than the tags associated with each
# thing. i.e. the key being the tag, instead of the key being the `thing` IDs.
tag_things:
  tag_app_development:
    - github_user_repo
    - localhost
  tag_deployment:
    - edge_aws_ecr_repo__aws_ecs_service__push

# `type`s we attach to `things` / `thing_dependencies` / `tags`, so they can be styled in common.
#
# This is like a tag, but doesn't require the user to click on the tag to apply the style.
# 
# Unlike tags, each `thing` / `thing_dependency` / `tag` can only have one `type`, so this map is keyed by the `thing` ID.
types:
  aws: "type_organisation"
  aws_iam: "type_service"
  aws_iam_ecs_policy: ~
  aws_ecr: "type_service"
  aws_ecr_repo: ~
  aws_ecr_repo_image_1: "type_docker_image"
  aws_ecr_repo_image_2: "type_docker_image"
  aws_ecs: "type_service"
  aws_ecs_cluster_app: ~
  aws_ecs_cluster_app_task: ~
  github: "type_organisation"
  github_user_repo: ~
  localhost: ~
  localhost_repo: ~
  localhost_repo_src: ~
  localhost_repo_target: ~
  localhost_repo_target_file_zip: ~
  localhost_repo_target_dist_dir: ~

  edge_localhost__github_user_repo__pull: ~
  edge_localhost__github_user_repo__push: ~
  edge_localhost__localhost__within: ~
  edge_github_user_repo__github_user_repo__within: ~
  edge_github_user_repo__aws_ecr_repo__push: ~
  edge_aws_ecr_repo__aws_ecs_service__push: ~

  tag_app_development: tag_type_default
  tag_deployment: tag_type_default

# Styles when the diagram has no user interaction.
#
# It's important for UX that the field nesting level (and hence indentation level) is consistent
# with the other `theme_*` data.
#
# `style_aliases` here are available to all the other `theme_*` data.
theme_default:
  # `StyleAliases` will have well-known keys, and is extendable to have custom keys.
  # 
  # i.e. a `StyleAlias` enum, with a final variant of `Custom(String)`.
  style_aliases:
    padding_none:
      padding_top: "0"
      padding_bottom: "0"
      padding_left: "0"
      padding_right: "0"
    padding_tight:
      padding_top: "2"
      padding_bottom: "2"
      padding_left: "2"
      padding_right: "2"
    padding_normal:
      padding_top: "4"
      padding_bottom: "4"
      padding_left: "4"
      padding_right: "4"
    padding_wide:
      padding_top: "6"
      padding_bottom: "6"
      padding_left: "6"
      padding_right: "6"
    shade_pale:
      fill_shade_hover: "50"
      fill_shade_normal: "100"
      fill_shade_focus: "200"
      fill_shade_active: "300"
      stroke_shade_hover: "100"
      stroke_shade_normal: "200"
      stroke_shade_focus: "300"
      stroke_shade_active: "400"
    shade_light:
      fill_shade_hover: "200"
      fill_shade_normal: "300"
      fill_shade_focus: "400"
      fill_shade_active: "500"
      stroke_shade_hover: "300"
      stroke_shade_normal: "400"
      stroke_shade_focus: "500"
      stroke_shade_active: "600"
    shade_medium:
      fill_shade_hover: "400"
      fill_shade_normal: "500"
      fill_shade_focus: "600"
      fill_shade_active: "700"
      stroke_shade_hover: "500"
      stroke_shade_normal: "600"
      stroke_shade_focus: "700"
      stroke_shade_active: "800"
    shade_dark:
      fill_shade_hover: "600"
      fill_shade_normal: "700"
      fill_shade_focus: "800"
      fill_shade_active: "900"
      stroke_shade_hover: "700"
      stroke_shade_normal: "800"
      stroke_shade_focus: "900"
      stroke_shade_active: "950"
    stroke_dashed_animated:
      stroke_style: "dashed"
      stroke_width: "2"
      animate: "[stroke-dashoffset-move_2s_linear_infinite]"

  # The keys in this map can be:
  #
  # * `thing_defaults`: Applies to all things.
  # * `edge_defaults`: Applies to all edges.
  # * `thing_id`: Applies to the particular thing.
  # * `edge_id`: Applies to the particular edge.
  # * `tag_id`: Applies to the tag.
  base_styles:
    thing_defaults:
      # Vector of style aliases to apply.
      style_aliases_applied: [shade_light]
      # Used for both fill and stroke colors.
      shape_color: "slate"
      stroke_style: "solid"
      stroke_width: "1"
      visibility: "visible"
    edge_defaults:
      stroke_width: "1"
      visibility: "visible"
    edge_localhost__github_user_repo__pull:
      style_aliases_applied: [shade_light]
      shape_color: "blue"

# Styles applied to things / edges of a particular `type` specified in `thing_types`.
theme_types_styles:
  type_organisation:
    thing_defaults:
      stroke_style: "dotted"
      style_aliases_applied: [shade_pale]

  type_docker_image:
    thing_defaults:
      shape_color: "blue"

# Styles when a `thing` is focused.
#
# Depending on which button is pressed, when a `thing` is focused, these same styles may be used to
# show:
#
# * Predecessors / successors linked to this `thing`.
# * Immediate dependencies vs transitive (maybe closest `n` neighbours).
theme_thing_dependencies_styles:
  things_excluded_styles:
    thing_defaults:
      visibility: "hidden"
    edge_defaults:
      visibility: "hidden
  things_included_styles:
    thing_defaults:
      visibility: "visible"

# When a tag is focused, things and edges associated with the tag are highlighted.
# 
# We also want to allow things that are not associated with the tag to be styled, but having one
# layer with the tag ID, and one layer of `things_included_styles` and `things_excluded_styles`
# makes it one nesting level deeper than the other `theme_*` keys.
# 
# So we have a `theme_tag_things_focus` map that applies to all tags' styles, and if the consumer
# wants to style things differently per tag, they can do so in the
# `theme_tag_things_focus_specific` map.
theme_tag_things_focus:
  things_included_styles:
    thing_defaults:
      opacity: "0.5"
  things_excluded_styles:
    thing_defaults:
      style_aliases_applied: [stroke_dashed_animated]

theme_tag_things_focus_specific:
  tag_app_development:
    thing_defaults:
      style_aliases_applied: [stroke_dashed_animated]

# Additional CSS to place in the SVG's inline `<styles>` section.
css: |-
  @keyframes stroke-dashoffset-move {
    0%   { stroke-dashoffset: 30; }
    100% { stroke-dashoffset: 0; }
  }
````

<object
    type="image/svg+xml"
    data="disposition/group_focus_experiment.svg"
    /></object>
<br/>
