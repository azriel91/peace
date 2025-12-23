# Example Input

````yaml
---
# Things in the diagram.
#
# This map defines the `ThingId`s and their display names.
things: &things
  t_aws: "☁️ Amazon Web Services"
  t_aws_iam: "🖊️ Identity and Access Management"
  t_aws_iam_ecs_policy: "🗒️ ECS IAM Policy"
  t_aws_ecr: "🗄️ Elastic Container Registry"
  t_aws_ecr_repo: "💽 web_app repo"
  t_aws_ecr_repo_image_1: "💿 Image 1"
  t_aws_ecr_repo_image_2: "💿 Image 2"
  t_aws_ecs: "💻 Elastic Container Service"
  t_aws_ecs_cluster_app: "🎛️ web_app cluster"
  t_aws_ecs_cluster_app_task: "🖨️ web_app task version 1"
  t_github: "🐙 GitHub"
  t_github_user_repo: "azriel91/web_app"
  t_localhost: "🧑‍💻 localhost"
  t_localhost_repo: "📂 ~/work/web_app"
  t_localhost_repo_src: "📝 src"
  t_localhost_repo_target: "📂 target"
  t_localhost_repo_target_file_zip: "📝 file.zip"
  t_localhost_repo_target_dist_dir: "📁 dist"

# Render a copy text button, and, when clicked, what text to place on the clipboard.
thing_copy_text:
  <<: *things
  t_localhost: "localhost"
  t_localhost_repo: "~/work/web_app"
  t_localhost_repo_src: "~/work/web_app/src"
  t_localhost_repo_target: "~/work/web_app/target"
  t_localhost_repo_target_file_zip: "~/work/web_app/target/file.zip"
  t_localhost_repo_target_dist_dir: "~/work/web_app/target/dist"

# Hierarchy of `thing`s.
#
# The `ThingHierarchy` is a tree structure stored as a map of `ThingId` to `ThingHierarchy`. This structure is strictly unidirectional.
#
# This defines the nesting, but perhaps we should use it to define the relative positioning as well.
#
# Do we want users to have control? Probably some, e.g. the order of declaration affects the
# position of the `thing` in a flex box.
#
# Other question, the positioning for a software dependency tree is different to the positioning
# for deployment topology. Maybe we allow the user to specify either "rank" based layout or "flow"
# based layout.
thing_hierarchy:
  t_aws:
    t_aws_iam:
      t_aws_iam_ecs_policy: {}
    t_aws_ecr:
      t_aws_ecr_repo:
        t_aws_ecr_repo_image_1: {}
        t_aws_ecr_repo_image_2: {}
    t_aws_ecs:
      t_aws_ecs_cluster_app:
        t_aws_ecs_cluster_app_task: {}

  t_github:
    t_github_user_repo: {}

  t_localhost:
    t_localhost_repo:
      t_localhost_repo_src: {}
      t_localhost_repo_target:
        t_localhost_repo_target_file_zip: {}
        t_localhost_repo_target_dist_dir: {}

# How to position things on the diagram.
#
# Not sure if this is the right approach yet, but ideas:
#
# * `rank`: `thing_dependencies`' edges affect how far a `thing` is from the beginning position.
# * `flex`: `thing_hierarchy` alternates between horizontal and vertical flex axes.
#
# ```yaml
# thing_layout:
#   # one of:
#   flex: "row"
#   # flex: "column"
#   # rank: "horizontal"
#   # rank: "vertical"
# ```

# Dependencies between things can be one way, or symmetric.
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
  edge_dep_t_localhost__t_github_user_repo__pull:
    # Cyclic dependencies are where the last thing in the list has an edge back to first thing.
    #
    # Must have at least 1 thing.
    cyclic:
      - t_localhost
      - t_github_user_repo
  edge_dep_t_localhost__t_github_user_repo__push:
    # Sequential dependencies are where there is an edge from each `thing` in the list to the next,
    # and no reverse edges.
    #
    # Must have at least 2 things.
    sequence:
      - t_localhost
      - t_github_user_repo
  edge_dep_t_localhost__t_localhost__within:
    cyclic:
      - t_localhost
  edge_dep_t_github_user_repo__t_github_user_repo__within:
    symmetric:
      - t_github_user_repo
  edge_dep_t_github_user_repo__t_aws_ecr_repo__push:
    sequence:
      - t_github_user_repo
      - t_aws_ecr_repo
  edge_dep_t_aws_ecr_repo__t_aws_ecs_service__push:
    sequence:
      - t_aws_ecr_repo
      - t_aws_ecs_service

# Interactions between things can be cyclic, a sequence, or symmetric.
#
# IDs here must not be the same as the ones in `thing_dependencies`, because the `entity_type`s'
# styling will collide.
#
# For 1 thing in a list, A, B, C, the following edges are added:
#
# * `cyclic`: `A -> A` (one edge from the node pointing back to itself).
# * `sequence`: none.
# * `symmetric`: `A -> A -> A` (two edges, one representing a request, one representing a response).
#
# For 3 things in a list, A, B, C, the following edges are added:
#
# * `cyclic`: `A -> B -> C -> A`.
# * `sequence`: `A -> B -> C`.
# * `symmetric`: `A -> B -> C -> B -> A`.
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
# We want to make it easy to define interactions between chains of things.
thing_interactions:
  edge_ix_t_localhost__t_github_user_repo__pull:
    # Symmetric interactions are where there is an edge from each `thing` in the list to the next,
    # and and reverse edges from the last thing in the list to the previous thing, until the first
    # thing in the list.
    #
    # Must have at least 1 thing.
    symmetric:
      - t_localhost
      - t_github_user_repo
  edge_ix_t_localhost__t_github_user_repo__push:
    # Sequential dependencies are where there is an edge from each `thing` in the list to the next,
    # and no reverse edges.
    #
    # Must have at least 2 things.
    sequence:
      - t_localhost
      - t_github_user_repo
  edge_ix_t_localhost__t_localhost__within:
    cyclic:
      - t_localhost
  edge_ix_t_github_user_repo__t_github_user_repo__within:
    symmetric:
      - t_github_user_repo
  edge_ix_t_github_user_repo__t_aws_ecr_repo__push:
    sequence:
      - t_github_user_repo
      - t_aws_ecr_repo
  edge_ix_t_aws_ecr_repo__t_aws_ecs_service__push:
    sequence:
      - t_aws_ecr_repo
      - t_aws_ecs_service

# Descriptions to render next to each node, edge group, or edge.
#
# This is intended to take markdown text.
#
# # Notes
#
# 1. Edge group IDs are from either `thing_dependencies` or `thing_interactions`.
# 2. Edge IDs are their edge group IDs, suffixed with "__" and the zero-based index of that edge in its group.
# 3. Descriptions for processes are not currently supported.
# 4. Descriptions for process steps are defined within the `process`'s `step_descs`.
entity_descs:
  # things
  t_localhost: "User's computer"

  # edge groups
  #
  # Shown when any of the edges in this group are focused.
  edge_ix_t_localhost__t_github_user_repo__pull: |-
    Fetch from GitHub
  edge_ix_t_localhost__t_github_user_repo__push: |-
    Push to GitHub

  # edges
  edge_ix_t_localhost__t_github_user_repo__pull__0: |-
    `git pull`
  edge_ix_t_localhost__t_github_user_repo__push__0: |-
    `git push`

# Processes are groupings of interactions between things sequenced over time.
#
# We want to make it easy to see which things are involved (in each step of) a process. By
# highlighting the things / edges when a user focuses on a step in a process, it brings clarity to
# the user.
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
      proc_app_dev_step_repository_clone: [edge_ix_t_localhost__t_github_user_repo__pull]
      proc_app_dev_step_project_build: [edge_ix_t_localhost__t_localhost__within]

  proc_app_release:
    name: "App Release"
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
      proc_app_release_step_crate_version_update: [edge_ix_t_localhost__t_localhost__within]
      proc_app_release_step_pull_request_open: [edge_ix_t_localhost__t_github_user_repo__pull]
      proc_app_release_step_tag_and_push: [edge_ix_t_localhost__t_github_user_repo__push]
      proc_app_release_step_gh_actions_build:
        [edge_ix_t_github_user_repo__t_github_user_repo__within]
      proc_app_release_step_gh_actions_publish: [edge_ix_t_github_user_repo__t_aws_ecr_repo__push]

  # Some processes not defined yet.
  #
  # proc_i12e_global_deploy: {}
  # proc_i12e_region_mgmt_deploy: {}
  # proc_i12e_region_tier_subnets_deploy: {}
  proc_i12e_region_tier_app_deploy:
    name: "Prod App Deploy"
    steps:
      proc_i12e_region_tier_app_deploy_step_ecs_service_update: "Update ECS service"

    step_descs:
      proc_i12e_region_tier_app_deploy_step_ecs_service_update: |-
        Deploy or update the existing ECS service with the new image.
    step_thing_interactions:
      proc_i12e_region_tier_app_deploy_step_ecs_service_update:
        [edge_ix_t_aws_ecr_repo__t_aws_ecs_service__push]

# Tags are labels that can be associated with things, so that the things can be highlighted when
# the tag is focused.
tags:
  tag_app_development: "Application Development"
  tag_deployment: "Deployment"

# Things associated with each tag.
#
# It probably makes sense to specify the `things` for each tag, than the tags associated with each
# thing. i.e. the key being the tag, instead of the key being the `thing` IDs.
tag_things:
  tag_app_development:
    - t_github_user_repo
    - t_localhost
  tag_deployment:
    - t_aws_ecr_repo
    - t_github_user_repo

# `Type`s are automatically attached to each entity:
#
# * `things`
# * `thing_dependencies`
# * `tags`
# * `processes`
# * `process_steps`
#
# These allow us to apply a common set of styles to the entities in the diagram with less
# duplication.
#
# Note: these do not actually appear in the diagram schema, but are listed so we know what default
# types are available.
#
# ```yaml
# types:
#   - type_thing_default
#   - type_tag_default
#   - type_process_default
#   - type_process_step_default
#   - type_dependency_edge_cyclic_default
#   - type_dependency_edge_cyclic_forward_default
#   - type_dependency_edge_sequence_default
#   - type_dependency_edge_sequence_forward_default
#   - type_dependency_edge_symmetric_default
#   - type_dependency_edge_symmetric_forward_default
#   - type_dependency_edge_symmetric_reverse_default
#   - type_interaction_edge_cyclic_default
#   - type_interaction_edge_cyclic_forward_default
#   - type_interaction_edge_sequence_default
#   - type_interaction_edge_sequence_forward_default
#   - type_interaction_edge_symmetric_default
#   - type_interaction_edge_symmetric_forward_default
#   - type_interaction_edge_symmetric_reverse_default
# ```
#
# Additional `type`s we attach to `things` / `thing_dependencies` / `tags`, so they can be styled
# in common.
#
# This is like a tag, but doesn't require the user to click on the tag to apply the style.
#
# Built-in types that are automatically attached to entities unless
# overridden:
#
# * `type_thing_default`
# * `type_tag_default`
# * `type_process_default`
# * `type_process_step_default`
#
# For edges, an edge group is generated for each dependency / interaction:
#
# Each edge group is assigned a type from the following:
#
# * `type_dependency_edge_sequence_default`
# * `type_dependency_edge_cyclic_default`
# * `type_dependency_edge_symmetric_default`
# * `type_interaction_edge_sequence_default`
# * `type_interaction_edge_cyclic_default`
# * `type_interaction_edge_symmetric_default`
#
# and each edge within each edge group is assigned a type from the following:
#
# * `type_dependency_edge_sequence_forward_default`
# * `type_dependency_edge_cyclic_forward_default`
# * `type_dependency_edge_symmetric_forward_default`
# * `type_dependency_edge_symmetric_reverse_default`
# * `type_interaction_edge_sequence_forward_default`
# * `type_interaction_edge_cyclic_forward_default`
# * `type_interaction_edge_symmetric_forward_default`
# * `type_interaction_edge_symmetric_reverse_default`
#
# The edge ID is the edge group ID specified in `thing_dependencies` /
# `thing_interactions`, suffixed with the zero-based index of the edge like
# so:
#
# ```text
# edge_id = edge_group_id + "__" + edge_index
# ```
#
# Additional entity types appended to each entity's default type.
# Each entity can have multiple types, allowing styles to be stacked.
#
# These types are appended to the entity's computed default type:
#
# - Things: `type_thing_default`
# - Tags: `type_tag_default`
# - Processes: `type_process_default`
# - Process steps: `type_process_step_default`
# - Edge groups: `type_dependency_edge_*_default` or type_interaction_edge_*_default
# - Edges: `type_dependency_edge_*_forward_default`, `type_interaction_edge_*_forward_default`, or `type_interaction_edge_*_reverse_default`
entity_types:
  t_aws:
    - type_organisation
  t_aws_iam:
    - type_service
  # t_aws_iam_ecs_policy: []
  t_aws_ecr:
    - type_service
  # t_aws_ecr_repo: []
  t_aws_ecr_repo_image_1:
    - type_docker_image
  t_aws_ecr_repo_image_2:
    - type_docker_image
  t_aws_ecs:
    - type_service
  # t_aws_ecs_cluster_app: []
  # t_aws_ecs_cluster_app_task: []
  t_github:
    - type_organisation
  # t_github_user_repo: []
  # t_localhost: []
  # t_localhost_repo: []
  # t_localhost_repo_src: []
  # t_localhost_repo_target: []
  # t_localhost_repo_target_file_zip: []
  # t_localhost_repo_target_dist_dir: []

  # edge_ix_t_localhost__t_github_user_repo__pull__0:
  #   - type_interaction_edge_symmetric_forward_default
  # edge_ix_t_localhost__t_github_user_repo__pull__1:
  #   - type_interaction_edge_symmetric_reverse_default
  # edge_ix_t_localhost__t_github_user_repo__push: []
  # edge_ix_t_localhost__t_localhost__within: []
  # edge_ix_t_github_user_repo__t_github_user_repo__within: []
  # edge_ix_t_github_user_repo__t_aws_ecr_repo__push: []
  # edge_ix_t_aws_ecr_repo__t_aws_ecs_service__push: []

  # tag_app_development:
  #   - tag_type_default
  # tag_deployment:
  #   - tag_type_default

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
      padding: "0.0"
      gap: "0.0"
    padding_tight:
      padding: "2.0"
      gap: "2.0"
    padding_normal:
      padding: "4.0"
      gap: "4.0"
    padding_wide:
      padding: "6.0"
      gap: "6.0"
    shade_pale:
      fill_shade_hover: "50"
      fill_shade_normal: "100"
      fill_shade_focus: "200"
      fill_shade_active: "300"
      stroke_shade_hover: "100"
      stroke_shade_normal: "200"
      stroke_shade_focus: "300"
      stroke_shade_active: "400"
      text_shade: "800"
    shade_light:
      fill_shade_hover: "200"
      fill_shade_normal: "300"
      fill_shade_focus: "400"
      fill_shade_active: "500"
      stroke_shade_hover: "300"
      stroke_shade_normal: "400"
      stroke_shade_focus: "500"
      stroke_shade_active: "600"
      text_shade: "900"
    shade_medium:
      fill_shade_hover: "400"
      fill_shade_normal: "500"
      fill_shade_focus: "600"
      fill_shade_active: "700"
      stroke_shade_hover: "500"
      stroke_shade_normal: "600"
      stroke_shade_focus: "700"
      stroke_shade_active: "800"
      text_shade: "950"
    shade_dark:
      fill_shade_hover: "600"
      fill_shade_normal: "700"
      fill_shade_focus: "800"
      fill_shade_active: "900"
      stroke_shade_hover: "700"
      stroke_shade_normal: "800"
      stroke_shade_focus: "900"
      stroke_shade_active: "950"
      text_shade: "950"
    stroke_dashed_animated:
      stroke_style: "dashed"
      stroke_width: "2"
      animate: "[stroke-dashoffset-move_2s_linear_infinite]"
    stroke_dashed_animated_request:
      animate: "[stroke-dashoffset-move-request_2s_linear_infinite]"
    stroke_dashed_animated_response:
      animate: "[stroke-dashoffset-move-response_2s_linear_infinite]"

  # The keys in this map can be:
  #
  # * `node_defaults`: Applies to all things.
  # * `edge_defaults`: Applies to all edges.
  # * `thing_id`: Applies to the particular thing.
  # * `edge_id`: Applies to the particular edge.
  # * `tag_id`: Applies to the tag.
  base_styles:
    node_defaults:
      # Vector of style aliases to apply.
      style_aliases_applied: [shade_light, padding_normal]
      # Used for both fill and stroke colors.
      shape_color: "slate"
      stroke_style: "solid"
      stroke_width: "1"
      text_color: "neutral"
      visibility: "visible"
    edge_defaults:
      text_color: "neutral"
    edge_ix_t_localhost__t_github_user_repo__pull:
      style_aliases_applied: [shade_light]
      shape_color: "blue"
    t_aws:
      shape_color: "yellow"
    t_github:
      shape_color: "neutral"

  process_step_selected_styles:
    node_defaults:
      style_aliases_applied: [shade_pale, stroke_dashed_animated]
    edge_defaults:
      visibility: "visible"

# Styles applied to things / edges of a particular `type` specified in `thing_types`.
theme_types_styles:
  # These are default styles that are built into `disposition`, but may be overridden by users.
  type_thing_default:
    node_defaults:
      style_aliases_applied: [shade_light]
      stroke_style: "solid"
      shape_color: "slate"
      stroke_width: "1"
  type_tag_default:
    node_defaults:
      style_aliases_applied: [shade_medium]
      stroke_style: "solid"
      shape_color: "emerald"
      stroke_width: "1"
  type_process_default:
    node_defaults:
      style_aliases_applied: [shade_medium]
      stroke_style: "solid"
      shape_color: "blue"
      stroke_width: "1"
  type_process_step_default:
    node_defaults:
      style_aliases_applied: [shade_medium]
      stroke_style: "solid"
      shape_color: "sky"
      stroke_width: "1"
      visibility: "invisible"

  # thing_dependencies - edge groups
  type_dependency_edge_sequence_default:
    edge_defaults:
      style_aliases_applied: [shade_dark]
      stroke_style: solid
      shape_color: "neutral"
      stroke_width: "1"
      visibility: "visible"
  type_dependency_edge_cyclic_default:
    edge_defaults:
      style_aliases_applied: [shade_dark]
      stroke_style: solid
      shape_color: "neutral"
      stroke_width: "1"
      visibility: "visible"
  type_dependency_edge_symmetric_default:
    edge_defaults:
      style_aliases_applied: [shade_dark]
      stroke_style: solid
      shape_color: "neutral"
      stroke_width: "1"
      visibility: "visible"

  # thing_dependencies - edges
  type_dependency_edge_sequence_forward_default:
    edge_defaults:
      stroke_width: "1"
  type_dependency_edge_cyclic_forward_default:
    edge_defaults:
      stroke_width: "1"
  type_dependency_edge_symmetric_forward_default:
    edge_defaults:
      stroke_width: "1"
  type_dependency_edge_symmetric_reverse_default:
    edge_defaults:
      stroke_width: "1"

  # thing_interactions - edge groups
  type_interaction_edge_sequence_default:
    edge_defaults:
      style_aliases_applied: [shade_dark]
      shape_color: "violet"
      stroke_width: "2"
      stroke_style: "dasharray:0,80,12,2,4,2,2,2,1,2,1,120"
      visibility: "invisible"
  type_interaction_edge_cyclic_default:
    edge_defaults:
      style_aliases_applied: [shade_dark]
      shape_color: "violet"
      stroke_width: "2"
      stroke_style: "dasharray:0,80,12,2,4,2,2,2,1,2,1,120"
      visibility: "invisible"
  type_interaction_edge_symmetric_default:
    edge_defaults:
      style_aliases_applied: [shade_dark]
      shape_color: "violet"
      stroke_width: "2"
      visibility: "invisible"

  # thing_interactions - edges
  type_interaction_edge_sequence_forward_default:
    edge_defaults:
      style_aliases_applied: [stroke_dashed_animated_request]
  type_interaction_edge_cyclic_forward_default:
    edge_defaults:
      style_aliases_applied: [stroke_dashed_animated_request]
  type_interaction_edge_symmetric_forward_default:
    edge_defaults:
      style_aliases_applied: [stroke_dashed_animated_request]
      stroke_style: "dasharray:0,80,12,2,4,2,2,2,1,2,1,120"
  type_interaction_edge_symmetric_reverse_default:
    edge_defaults:
      style_aliases_applied: [stroke_dashed_animated_response]
      stroke_style: "dasharray:0,120,1,2,1,2,2,2,4,2,8,2,20,80"

  # custom styles that users can provide
  type_organisation:
    node_defaults:
      style_aliases_applied: [shade_pale]
      stroke_style: "dotted"
  type_service:
    node_defaults:
      stroke_style: "dashed"

  type_docker_image:
    node_defaults:
      shape_color: "sky"

# Styles when a `thing` is focused.
#
# Depending on which button is pressed, when a `thing` is focused, these same styles may be used to
# show:
#
# * Predecessors / successors linked to this `thing`.
# * Immediate dependencies vs transitive (maybe closest `n` neighbours).
theme_thing_dependencies_styles:
  things_excluded_styles:
    node_defaults:
      visibility: "hidden"
    edge_defaults:
      visibility: "hidden"
  things_included_styles:
    node_defaults:
      visibility: "visible"

# When a tag is focused, things and edges associated with the tag are highlighted.
#
# We also want to allow things that are not associated with the tag to be styled, but having one
# layer with the tag ID, and one layer of `things_included_styles` and `things_excluded_styles`
# makes it one nesting level deeper than the other `theme_*` keys.
#
# There is a `tag_defaults` key that applies to all tags' styles, and if the user wants to style things differently per tag, they can specify them per tag ID.
theme_tag_things_focus:
  tag_defaults:
    node_defaults:
      style_aliases_applied: [shade_pale, stroke_dashed_animated]
    node_excluded_defaults:
      opacity: "75"

  tag_app_development:
    node_excluded_defaults:
      opacity: "50"
    node_defaults:
      style_aliases_applied: [stroke_dashed_animated]

# Additional CSS to place in the SVG's inline `<styles>` section.
css: |-
  @keyframes stroke-dashoffset-move {
    0%   { stroke-dasharray: 3; stroke-dashoffset: 30; }
    100% { stroke-dasharray: 3; stroke-dashoffset: 0; }
  }
  @keyframes stroke-dashoffset-move-request {
    0%   { stroke-dashoffset: 0; }
    100% { stroke-dashoffset: 228; }
  }
  @keyframes stroke-dashoffset-move-response {
    0%   { stroke-dashoffset: 0; }
    100% { stroke-dashoffset: -248; }
  }
````
