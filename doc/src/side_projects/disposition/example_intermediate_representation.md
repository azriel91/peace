# Example Intermediate Representation

This is the computed data structure from combining the layered values from the input data.

````yaml
# Everything that needs to be represented as a `node` on the diagram, including:
#
# * `things`
# * `tags`
# * `processes`
# * `steps`
nodes: &nodes
  # things
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
  t_localhost: "🧑‍💻 Localhost"
  t_localhost_repo: "📂 ~/work/web_app"
  t_localhost_repo_src: "📝 src"
  t_localhost_repo_target: "📂 target"
  t_localhost_repo_target_file_zip: "📝 file.zip"
  t_localhost_repo_target_dist_dir: "📁 dist"

  # tags
  tag_app_development: "Application Development"
  tag_deployment: "Deployment"

  # processes
  proc_app_dev: "App Development"
  proc_app_release: "App Release"
  proc_i12e_region_tier_app_deploy: "Prod App Deploy"

  # steps
  proc_app_dev_step_repository_clone: "Clone repository"
  proc_app_dev_step_project_build: "Build project"

  proc_app_release_step_crate_version_update: "Update crate versions"
  proc_app_release_step_pull_request_open: "Open PR"
  proc_app_release_step_tag_and_push: "Tag and push"
  proc_app_release_step_gh_actions_build: "Github Actions build"
  proc_app_release_step_gh_actions_publish: "Github Actions publish"

  proc_i12e_region_tier_app_deploy_step_ecs_service_update: "Update ECS service"

# Render a copy text button, and, when clicked, what text to place on the clipboard.
#
# This differs from the input schema by only including the `thing`s, not all `node`s.
#
# Note that all nodes' text will still be selectable and copyable with the regular hotkeys.
node_copy_text:
  # things
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
  t_localhost: "🧑‍💻 Localhost"
  t_localhost_repo: "📂 ~/work/web_app"
  t_localhost_repo_src: "📝 src"
  t_localhost_repo_target: "📂 target"
  t_localhost_repo_target_file_zip: "📝 file.zip"
  t_localhost_repo_target_dist_dir: "📁 dist"

  t_localhost_repo: "~/work/web_app"
  t_localhost_repo_src: "~/work/web_app/src"
  t_localhost_repo_target: "~/work/web_app/target"
  t_localhost_repo_target_file_zip: "~/work/web_app/target/file.zip"
  t_localhost_repo_target_dist_dir: "~/work/web_app/target/dist"

# Rich level of detail for a given node.
node_descs:
  # process steps
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
  proc_i12e_region_tier_app_deploy_step_ecs_service_update: |-
    Deploy or update the existing ECS service with the new image.

# Hierarchy of all nodes, to be laid out based on `node_layout`.
#
# This should be roughly the same as the DOM element hierarchy, which has the following constraints
# over the input schema:
#
# * `process` nodes have to come before `thing` nodes in order for the sibling selector (tailwind
#   peers) to work.
# * `process_step` nodes have to be descendents of `process` nodes, to allow the `process` nodes to
#    be visible when the `process_step` nodes have focus (via `focus-within`).
# * `thing` nodes may have wrapping nodes, so that we don't exceed the limit of CSS classes per
#   element.
#
# One difference between the input schema and the IR schema is, the IR schema needs to specify the
# hierarchy for nodes that are not only `thing`s, e.g. `process` and `process_step`s.
#
# ⚠️ The order of node declaration is important -- `process` nodes must come earlier than `thing`
# nodes in the DOM structure for the peer/sibling CSS selectors to work correctly. *However*,
# visually the `process` nodes may be placed to the right of the `thing` nodes.
#
# The DOM structure *may* also differ in this way: just because the `node_hierarchy` indicates
# nesting, does not mean the actual DOM hierarchy must have its elements nested. i.e. they may be
# siblings, just positioned and sized such that they visually appear to be nested.
node_hierarchy:
  # Tags before everything else (required for peer selector to target processes/things/edges)
  tag_app_development: {}
  tag_deployment: {}

  # Processes before things/edges (required for peer selector to target things/edges)
  proc_app_dev:
    proc_app_dev_step_repository_clone: {}
    proc_app_dev_step_project_build: {}
  proc_app_release:
    proc_app_release_step_crate_version_update: {}
    proc_app_release_step_pull_request_open: {}
    proc_app_release_step_tag_and_push: {}
    proc_app_release_step_gh_actions_build: {}
    proc_app_release_step_gh_actions_publish: {}
  proc_i12e_region_tier_app_deploy:
    proc_i12e_region_tier_app_deploy_step_ecs_service_update: {}

  # Things (same hierarchy as input `thing_hierarchy`)
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

# Edges derived from `thing_dependencies` and `thing_interactions`.
#
# Each edge has:
# - `from`: source node ID
# - `to`: target node ID
edge_groups:
  edge_t_localhost__t_github_user_repo:
    - from: t_github_user_repo
      to: t_localhost
    - from: t_localhost
      to: t_github_user_repo
  edge_t_localhost__t_github_user_repo__push:
    - from: t_localhost
      to: t_github_user_repo
  edge_t_localhost__t_localhost__within:
    - from: t_localhost
      to: t_localhost
  edge_t_github_user_repo__t_github_user_repo__within:
    - from: t_github_user_repo
      to: t_github_user_repo
  edge_t_github_user_repo__t_aws_ecr_repo__push:
    - from: t_github_user_repo
      to: t_aws_ecr_repo
  edge_t_aws_ecr_repo__t_aws_ecs_service__push:
    - from: t_aws_ecr_repo
      to: t_aws_ecs_cluster_app_task

# Text placed next to the edges.
#
# We may need to factor these in as a DOM node so that there is space reserved for these descriptions alongside the `node` DOM nodes.
edge_group_descs:
  edge_t_localhost__t_github_user_repo__pull: "`git pull`"
  edge_t_localhost__t_github_user_repo__push: "`git push`"
  edge_t_localhost__t_localhost__within: ~
  edge_t_github_user_repo__t_github_user_repo__within: ~
  edge_t_github_user_repo__t_aws_ecr_repo__push: ~
  edge_t_aws_ecr_repo__t_aws_ecs_service__push: ~

entity_types:
  # things
  t_aws: [type_thing_default, type_organisation]
  t_aws_iam: [type_thing_default, type_service]
  t_aws_iam_ecs_policy: [type_thing_default]
  t_aws_ecr: [type_thing_default, type_service]
  t_aws_ecr_repo: [type_thing_default]
  t_aws_ecr_repo_image_1: [type_thing_default, type_docker_image]
  t_aws_ecr_repo_image_2: [type_thing_default, type_docker_image]
  t_aws_ecs: [type_thing_default, type_service]
  t_aws_ecs_cluster_app: [type_thing_default]
  t_aws_ecs_cluster_app_task: [type_thing_default]
  t_github: [type_thing_default, type_organisation]
  t_github_user_repo: [type_thing_default]
  t_localhost: [type_thing_default]
  t_localhost_repo: [type_thing_default]
  t_localhost_repo_src: [type_thing_default]
  t_localhost_repo_target: [type_thing_default]
  t_localhost_repo_target_file_zip: [type_thing_default]
  t_localhost_repo_target_dist_dir: [type_thing_default]

  # tags
  tag_app_development: [tag_type_default]
  tag_deployment: [tag_type_default]

  # processes
  proc_app_dev: [type_process_default]
  proc_app_release: [type_process_default]
  proc_i12e_region_tier_app_deploy: [type_process_default]

  # process steps
  proc_app_dev_step_repository_clone: [type_process_step_default]
  proc_app_dev_step_project_build: [type_process_step_default]

  proc_app_release_step_crate_version_update: [type_process_step_default]
  proc_app_release_step_pull_request_open: [type_process_step_default]
  proc_app_release_step_tag_and_push: [type_process_step_default]
  proc_app_release_step_gh_actions_build: [type_process_step_default]
  proc_app_release_step_gh_actions_publish: [type_process_step_default]

  proc_i12e_region_tier_app_deploy_step_ecs_service_update: [type_process_step_default]

  # edges
  edge_t_localhost__t_github_user_repo__pull: [type_edge_dependency_cyclic_default, type_edge_interaction_cyclic_default]
  edge_t_localhost__t_github_user_repo__push: [type_edge_dependency_sequence_default, type_edge_interaction_sequence_default]
  edge_t_localhost__t_localhost__within: [type_edge_dependency_cyclic_default, type_edge_interaction_cyclic_default]
  edge_t_github_user_repo__t_github_user_repo__within: [type_edge_dependency_cyclic_default, type_edge_interaction_cyclic_default]
  edge_t_github_user_repo__t_aws_ecr_repo__push: [type_edge_dependency_sequence_default, type_edge_interaction_sequence_default]
  edge_t_aws_ecr_repo__t_aws_ecs_service__push: [type_edge_dependency_sequence_default, type_edge_interaction_sequence_default]

# Tailwind CSS classes for interactive visibility behaviour.
#
# ## Visibility Patterns
#
# 1. Process -> Process Steps visibility:
#
#     * Process node: `group/{process_id}` class
#     * Process steps: `invisible group-focus-within/{process_id}:visible`
#     * When process (or any child) has focus, steps become and remain visible
#
# 2. Process Step -> Edges visibility:
#
#     * Process step: `peer/{step_id}` class
#     * Edges: `invisible peer-focus/{step_id}:visible`
#     * Edges must be DOM siblings AFTER the step element
#
# 3. **Alternative:** `:target` based visibility:
#
#     * When element ID matches URL fragment (e.g. `#step_id`)
#     * Use `invisible target:visible` on the element
#     * Use `[&:has(~_#step_id:target)]:visible` on edges
#     * Use `peer-[:where([data-step='3']):target]:visible` on edges
#
# # Notes
#
# * `[stroke-dasharray:2]` in an SVG approximates `border-dotted` on HTML elements.
tailwind_classes:
  # Processes - act as group containers for their steps
  proc_app_dev: >-
    group/proc_app_dev
  proc_app_release: >-
    group/proc_app_release
  proc_i12e_region_tier_app_deploy: >-
    group/proc_i12e_region_tier_app_deploy

  # Process steps - visible when parent process has focus, act as peers for edges
  proc_app_dev_step_repository_clone: >-
    peer/proc_app_dev_step_repository_clone
    invisible
    group-focus-within/proc_app_dev:visible
  proc_app_dev_step_project_build: >-
    peer/proc_app_dev_step_project_build
    invisible
    group-focus-within/proc_app_dev:visible

  proc_app_release_step_crate_version_update: >-
    peer/proc_app_release_step_crate_version_update
    invisible
    group-focus-within/proc_app_release:visible
  proc_app_release_step_pull_request_open: >-
    peer/proc_app_release_step_pull_request_open
    invisible
    group-focus-within/proc_app_release:visible
  proc_app_release_step_tag_and_push: >-
    peer/proc_app_release_step_tag_and_push
    invisible
    group-focus-within/proc_app_release:visible
  proc_app_release_step_gh_actions_build: >-
    peer/proc_app_release_step_gh_actions_build
    invisible
    group-focus-within/proc_app_release:visible
  proc_app_release_step_gh_actions_publish: >-
    peer/proc_app_release_step_gh_actions_publish
    invisible
    group-focus-within/proc_app_release:visible

  proc_i12e_region_tier_app_deploy_step_ecs_service_update: >-
    peer/proc_i12e_region_tier_app_deploy_step_ecs_service_update
    invisible
    group-focus-within/proc_i12e_region_tier_app_deploy:visible

  # Edges - visible when their associated step has focus
  # Maps edge_id -> list of step_ids that trigger visibility
  edge_t_localhost__t_github_user_repo__pull: >-
    invisible
    peer-focus/proc_app_dev_step_repository_clone:visible
    peer-focus/proc_app_release_step_pull_request_open:visible
  edge_t_localhost__t_github_user_repo__push: >-
    invisible
    peer-focus/proc_app_release_step_tag_and_push:visible
  edge_t_localhost__t_localhost__within: >-
    invisible
    peer-focus/proc_app_dev_step_project_build:visible
    peer-focus/proc_app_release_step_crate_version_update:visible
  edge_t_github_user_repo__t_github_user_repo__within: >-
    invisible
    peer-focus/proc_app_release_step_gh_actions_build:visible
  edge_t_github_user_repo__t_aws_ecr_repo__push: >-
    invisible
    peer-focus/proc_app_release_step_gh_actions_publish:visible
  edge_t_aws_ecr_repo__t_aws_ecs_service__push: >-
    invisible
    peer-focus/proc_i12e_region_tier_app_deploy_step_ecs_service_update:visible

  # Styles for things are as follows:
  #
  # For the inner `<path>` element:
  #
  # * `hover:stroke-{color}-{shade}` when hovered.
  # * `stroke-{color}-{shade}` for normal state.
  # * `focus:stroke-{color}-{shade}` when focused.
  # * `active:stroke-{color}-{shade}` when pressed.
  # * `hover:fill-{color}-{shade}` when hovered.
  # * `fill-{color}-{shade}` for normal state.
  # * `focus:fill-{color}-{shade}` when focused.
  # * `active:fill-{color}-{shade}` when pressed.
  #
  # For the inner `<text>` element:
  #
  # * `[&>text]:fill-{color}-{shade}` for text in all states.
  #
  # The following classes are used to highlight the `thing`'s `<path>` when a related process step
  # is focused:
  #
  # * `animate-[stroke-dashoffset-move_2s_linear_infinite]`
  # * `stroke-{color}-{shade}` where `shade` is the "normal" shade, but brighter by one level.
  # * `fill-{color}-{shade}` where `shade` is the "normal" shade, but brighter by one level.
  #
  # They are applied to each thing, just with the appropriate `peer-focus/{process_step}` prefix.
  #
  # We could collapse the `stroke-dashoffset` into the animation, but we can't collapse the `shade`
  # because they are part of the stroke and fill colors -- we have to choose between duplication
  # either here, or in animations.
  t_aws: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_aws_iam: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_aws_iam_ecs_policy: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_aws_ecr: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_aws_ecr_repo: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950

    peer-focus/proc_app_release_step_gh_actions_publish:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_release_step_gh_actions_publish:stroke-slate-500
    peer-focus/proc_app_release_step_gh_actions_publish:fill-slate-100

    peer-focus/proc_i12e_region_tier_app_deploy_step_ecs_service_update:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_i12e_region_tier_app_deploy_step_ecs_service_update:stroke-slate-500
    peer-focus/proc_i12e_region_tier_app_deploy_step_ecs_service_update:fill-slate-100

  t_aws_ecr_repo_image_1: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_aws_ecr_repo_image_2: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_aws_ecs: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_aws_ecs_cluster_app: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_aws_ecs_cluster_app_task: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_github: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_github_user_repo: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950

    peer-focus/proc_app_dev_step_repository_clone:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_dev_step_repository_clone:stroke-slate-500
    peer-focus/proc_app_dev_step_repository_clone:fill-slate-100

    peer-focus/proc_app_release_step_pull_request_open:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_release_step_pull_request_open:stroke-slate-500
    peer-focus/proc_app_release_step_pull_request_open:fill-slate-100

    peer-focus/proc_app_release_step_tag_and_push:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_release_step_tag_and_push:stroke-slate-500
    peer-focus/proc_app_release_step_tag_and_push:fill-slate-100

    peer-focus/proc_app_release_step_gh_actions_build:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_release_step_gh_actions_build:stroke-slate-500
    peer-focus/proc_app_release_step_gh_actions_build:fill-slate-100

    peer-focus/proc_app_release_step_gh_actions_publish:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_release_step_gh_actions_publish:stroke-slate-500
    peer-focus/proc_app_release_step_gh_actions_publish:fill-slate-100

  t_localhost: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950

    peer-focus/proc_app_dev_step_repository_clone:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_dev_step_repository_clone:stroke-slate-500
    peer-focus/proc_app_dev_step_repository_clone:fill-slate-100

    peer-focus/proc_app_release_step_pull_request_open:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_release_step_pull_request_open:stroke-slate-500
    peer-focus/proc_app_release_step_pull_request_open:fill-slate-100

    peer-focus/proc_app_release_step_tag_and_push:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_release_step_tag_and_push:stroke-slate-500
    peer-focus/proc_app_release_step_tag_and_push:fill-slate-100

    peer-focus/proc_app_dev_step_project_build:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_dev_step_project_build:stroke-slate-500
    peer-focus/proc_app_dev_step_project_build:fill-slate-100

    peer-focus/proc_app_release_step_crate_version_update:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-focus/proc_app_release_step_crate_version_update:stroke-slate-500
    peer-focus/proc_app_release_step_crate_version_update:fill-slate-100

  t_localhost_repo: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_localhost_repo_src: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_localhost_repo_target: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_localhost_repo_target_file_zip: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950
  t_localhost_repo_target_dist_dir: >-
    stroke-1
    visible
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    [&>text]:fill-slate-950

# Layout configuration for each node.
#
# Variants:
#
# * `flex`: { direction: "row" | "row_reverse" | "column" | "column_reverse", wrap: bool, gap: string }
# * `none`: (no children to lay out)
#
# The layout cascades: if not specified, inherits from parent or uses default.
node_layout:
  # Root container
  #
  # This is in `column_reverse` order so tag node state controls `things` and `processes`:
  #
  # ```text
  # .------------------------------------.
  # | 1. _things_and_processes_container |
  # | -----------------------------------|
  # | 0. _tags_container                 |
  # '------------------------------------'
  # ```
  _root:
    flex:
      direction: "column_reverse"
      wrap: true
      gap: "4"

  # Things and Processes container.
  #
  # This is in `row_reverse` order so process node state controls `things`:
  #
  # ```text
  # .------------------------------------------------.
  # | 1. _things_container | 0. _processes_container |
  # '------------------------------------------------'
  # ```
  _things_and_processes_container:
    flex:
      direction: "row_reverse"
      wrap: true
      gap: "4"

  # Processes container (groups all processes horizontally)
  #
  # This is in `row` order so processs are laid out left to right:
  #
  # ```text
  # .-----------------------------------------------------------------------------.
  # | 0. proc_app_dev | 1. proc_app_release | 2. proc_i12e_region_tier_app_deploy |
  # '-----------------------------------------------------------------------------'
  # ```
  _processes_container:
    flex:
      direction: "row"
      wrap: true
      gap: "4"

  # Individual processes (steps stacked vertically)
  #
  # This is in `column` order so steps are laid out top to bottom:
  #
  # ```text
  # .---------------------------------------.
  # | 0. proc_app_dev_step_repository_clone |
  # |---------------------------------------|
  # | 1. proc_app_dev_step_project_build    |
  # '---------------------------------------'
  # ```
  proc_app_dev:
    flex:
      direction: "column"
      wrap: false
      gap: "2"
  proc_app_release:
    flex:
      direction: "column"
      wrap: false
      gap: "2"
  proc_i12e_region_tier_app_deploy:
    flex:
      direction: "column"
      wrap: false
      gap: "2"

  # Process steps are leaves
  proc_app_dev_step_repository_clone: none
  proc_app_dev_step_project_build: none
  proc_app_release_step_crate_version_update: none
  proc_app_release_step_pull_request_open: none
  proc_app_release_step_tag_and_push: none
  proc_app_release_step_gh_actions_build: none
  proc_app_release_step_gh_actions_publish: none
  proc_i12e_region_tier_app_deploy_step_ecs_service_update: none

  # Tags container
  #
  # This is in `row` order so tags are laid out left to right:
  #
  # ```text
  # .--------------------------------------------.
  # | 0. tag_app_development | 1. tag_deployment |
  # '--------------------------------------------'
  # ```
  _tags_container:
    flex:
      direction: "row"
      wrap: true
      gap: "2"

  # Tags are leaves
  tag_app_development: none
  tag_deployment: none

  # Things container (top-level things arranged in a row)
  _things_container:
    flex:
      direction: "row"
      wrap: true
      gap: "4"

  # Top-level things (children alternate between columns and rows)
  t_aws:
    flex:
      direction: "column"
      wrap: false
      gap: "2"
  t_aws_iam:
    flex:
      direction: "row"
      wrap: false
      gap: "1"
  t_aws_iam_ecs_policy: none
  t_aws_ecr:
    flex:
      direction: "row"
      wrap: false
      gap: "1"
  t_aws_ecr_repo:
    flex:
      direction: "column"
      wrap: true
      gap: "1"
  t_aws_ecr_repo_image_1: none
  t_aws_ecr_repo_image_2: none
  t_aws_ecs:
    flex:
      direction: "row"
      wrap: false
      gap: "1"
  t_aws_ecs_cluster_app:
    flex:
      direction: "column"
      wrap: false
      gap: "1"
  t_aws_ecs_cluster_app_task: none

  t_github:
    flex:
      direction: "row"
      wrap: false
      gap: "2"
  t_github_user_repo: none

  t_localhost:
    flex:
      direction: "row"
      wrap: false
      gap: "2"
  t_localhost_repo:
    flex:
      direction: "column"
      wrap: false
      gap: "1"
  t_localhost_repo_src: none
  t_localhost_repo_target:
    flex:
      direction: "row"
      wrap: true
      gap: "1"
  t_localhost_repo_target_file_zip: none
  t_localhost_repo_target_dist_dir: none

# Types specify which template styles to use for each:
#
# * `node`
# * `edge`
#
# This should be a `Set<TypeId>` as `Type`s should be stackable.
entity_types:
  # things
  t_aws: ["type_thing_default"]
  t_aws_iam: ["type_thing_default"]
  t_aws_iam_ecs_policy: ["type_thing_default"]
  t_aws_ecr: ["type_thing_default"]
  t_aws_ecr_repo: ["type_thing_default"]
  t_aws_ecr_repo_image_1: ["type_thing_default"]
  t_aws_ecr_repo_image_2: ["type_thing_default"]
  t_aws_ecs: ["type_thing_default"]
  t_aws_ecs_cluster_app: ["type_thing_default"]
  t_aws_ecs_cluster_app_task: ["type_thing_default"]
  t_github: ["type_thing_default"]
  t_github_user_repo: ["type_thing_default"]
  t_localhost: ["type_thing_default"]
  t_localhost_repo: ["type_thing_default"]
  t_localhost_repo_src: ["type_thing_default"]
  t_localhost_repo_target: ["type_thing_default"]
  t_localhost_repo_target_file_zip: ["type_thing_default"]
  t_localhost_repo_target_dist_dir: ["type_thing_default"]

  # tags
  tag_app_development: ["type_tag_default"]
  tag_deployment: ["type_tag_default"]

  # processes
  proc_app_dev: ["type_process_default"]
  proc_app_release: ["type_process_default"]
  proc_i12e_region_tier_app_deploy: ["type_process_default"]

  # steps
  proc_app_dev_step_repository_clone: ["type_process_step_default"]
  proc_app_dev_step_project_build: ["type_process_step_default"]

  proc_app_release_step_crate_version_update: ["type_process_step_default"]
  proc_app_release_step_pull_request_open: ["type_process_step_default"]
  proc_app_release_step_tag_and_push: ["type_process_step_default"]
  proc_app_release_step_gh_actions_build: ["type_process_step_default"]
  proc_app_release_step_gh_actions_publish: ["type_process_step_default"]

  proc_i12e_region_tier_app_deploy_step_ecs_service_update: "type_process_step_default"

css: >-
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
