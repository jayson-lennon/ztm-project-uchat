-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler version: 1.0.1
-- PostgreSQL version: 15.0
-- Project Site: pgmodeler.io
-- Model Author: ---

-- Database creation must be performed outside a multi lined SQL file. 
-- These commands were put in this file only as a convenience.
-- 
-- object: uchat | type: DATABASE --
-- ddl-end --


-- object: public.posts | type: TABLE --
-- DROP TABLE IF EXISTS public.posts CASCADE;
CREATE TABLE public.posts (
  id uuid NOT NULL,
  user_id uuid NOT NULL,
  content jsonb NOT NULL,
  time_posted timestamptz NOT NULL,
  direct_message_to uuid,
  reply_to uuid,
  created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT posts_pk PRIMARY KEY (id)
);
-- ddl-end --
COMMENT ON COLUMN public.posts.time_posted IS E'when time is in the future, the post is scheduled for that time';
-- ddl-end --

-- object: public.poll_choices | type: TABLE --
-- DROP TABLE IF EXISTS public.poll_choices CASCADE;
CREATE TABLE public.poll_choices (
  id uuid NOT NULL,
  choice text NOT NULL,
  post_id uuid NOT NULL,
  CONSTRAINT poll_choices_pk PRIMARY KEY (id)
);
-- ddl-end --

-- object: public.poll_votes | type: TABLE --
-- DROP TABLE IF EXISTS public.poll_votes CASCADE;
CREATE TABLE public.poll_votes (
  user_id uuid NOT NULL,
  post_id uuid NOT NULL,
  choice_id uuid NOT NULL,
  created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT poll_votes_pk PRIMARY KEY (user_id,post_id)
);
-- ddl-end --

-- object: public.reactions | type: TABLE --
-- DROP TABLE IF EXISTS public.reactions CASCADE;
CREATE TABLE public.reactions (
  user_id uuid NOT NULL,
  post_id uuid NOT NULL,
  created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
  like_status smallint NOT NULL,
  reaction jsonb,
  CONSTRAINT reactions_pk PRIMARY KEY (user_id,post_id)
);
-- ddl-end --

-- object: public.bookmarks | type: TABLE --
-- DROP TABLE IF EXISTS public.bookmarks CASCADE;
CREATE TABLE public.bookmarks (
  user_id uuid NOT NULL,
  post_id uuid NOT NULL,
  created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT bookmarks_pk PRIMARY KEY (user_id,post_id)
);
-- ddl-end --

-- object: public.users | type: TABLE --
-- DROP TABLE IF EXISTS public.users CASCADE;
CREATE TABLE public.users (
  id uuid NOT NULL,
  email text,
  email_confirmed timestamptz,
  password_hash text NOT NULL,
  display_name text,
  handle text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
  profile_image text,
  CONSTRAINT email_is_unique UNIQUE (email),
  CONSTRAINT users_pk PRIMARY KEY (id),
  CONSTRAINT handle_is_unique UNIQUE (handle)
);
-- ddl-end --

-- object: public.web | type: TABLE --
-- DROP TABLE IF EXISTS public.web CASCADE;
CREATE TABLE public.web (
  id uuid NOT NULL,
  user_id uuid NOT NULL,
  expires_at timestamptz NOT NULL,
  created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
  fingerprint jsonb NOT NULL,
  CONSTRAINT web_pk PRIMARY KEY (id),
  CONSTRAINT one_session_per_device UNIQUE (user_id,fingerprint)
);
-- ddl-end --

-- object: public.followers | type: TABLE --
-- DROP TABLE IF EXISTS public.followers CASCADE;
CREATE TABLE public.followers (
  user_id uuid NOT NULL,
  follows uuid NOT NULL,
  created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT followers_pk PRIMARY KEY (user_id,follows)
);
-- ddl-end --

-- object: post_pagination_index | type: INDEX --
-- DROP INDEX IF EXISTS public.post_pagination_index CASCADE;
CREATE INDEX post_pagination_index ON public.posts
USING btree
(
  user_id,
  time_posted
)
INCLUDE (id,user_id);
-- ddl-end --

-- object: public.boosts | type: TABLE --
-- DROP TABLE IF EXISTS public.boosts CASCADE;
CREATE TABLE public.boosts (
  post_id uuid NOT NULL,
  user_id uuid NOT NULL,
  boosted_at timestamptz NOT NULL,
  CONSTRAINT boosts_pk PRIMARY KEY (post_id,user_id)
);
-- ddl-end --

-- object: user_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.posts DROP CONSTRAINT IF EXISTS user_id_fk CASCADE;
ALTER TABLE public.posts ADD CONSTRAINT user_id_fk FOREIGN KEY (user_id)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: direct_message_fk | type: CONSTRAINT --
-- ALTER TABLE public.posts DROP CONSTRAINT IF EXISTS direct_message_fk CASCADE;
ALTER TABLE public.posts ADD CONSTRAINT direct_message_fk FOREIGN KEY (direct_message_to)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: comment_fk | type: CONSTRAINT --
-- ALTER TABLE public.posts DROP CONSTRAINT IF EXISTS comment_fk CASCADE;
ALTER TABLE public.posts ADD CONSTRAINT comment_fk FOREIGN KEY (reply_to)
REFERENCES public.posts (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: post_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.poll_choices DROP CONSTRAINT IF EXISTS post_id_fk CASCADE;
ALTER TABLE public.poll_choices ADD CONSTRAINT post_id_fk FOREIGN KEY (post_id)
REFERENCES public.posts (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: post_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.poll_votes DROP CONSTRAINT IF EXISTS post_id_fk CASCADE;
ALTER TABLE public.poll_votes ADD CONSTRAINT post_id_fk FOREIGN KEY (post_id)
REFERENCES public.posts (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: user_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.poll_votes DROP CONSTRAINT IF EXISTS user_id_fk CASCADE;
ALTER TABLE public.poll_votes ADD CONSTRAINT user_id_fk FOREIGN KEY (user_id)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: choice_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.poll_votes DROP CONSTRAINT IF EXISTS choice_id_fk CASCADE;
ALTER TABLE public.poll_votes ADD CONSTRAINT choice_id_fk FOREIGN KEY (choice_id)
REFERENCES public.poll_choices (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: post_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.reactions DROP CONSTRAINT IF EXISTS post_id_fk CASCADE;
ALTER TABLE public.reactions ADD CONSTRAINT post_id_fk FOREIGN KEY (post_id)
REFERENCES public.posts (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: user_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.reactions DROP CONSTRAINT IF EXISTS user_id_fk CASCADE;
ALTER TABLE public.reactions ADD CONSTRAINT user_id_fk FOREIGN KEY (user_id)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: post_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.bookmarks DROP CONSTRAINT IF EXISTS post_id_fk CASCADE;
ALTER TABLE public.bookmarks ADD CONSTRAINT post_id_fk FOREIGN KEY (post_id)
REFERENCES public.posts (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: user_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.bookmarks DROP CONSTRAINT IF EXISTS user_id_fk CASCADE;
ALTER TABLE public.bookmarks ADD CONSTRAINT user_id_fk FOREIGN KEY (user_id)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: user_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.web DROP CONSTRAINT IF EXISTS user_id_fk CASCADE;
ALTER TABLE public.web ADD CONSTRAINT user_id_fk FOREIGN KEY (user_id)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: user_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.followers DROP CONSTRAINT IF EXISTS user_id_fk CASCADE;
ALTER TABLE public.followers ADD CONSTRAINT user_id_fk FOREIGN KEY (user_id)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: follows_user_id | type: CONSTRAINT --
-- ALTER TABLE public.followers DROP CONSTRAINT IF EXISTS follows_user_id CASCADE;
ALTER TABLE public.followers ADD CONSTRAINT follows_user_id FOREIGN KEY (follows)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: post_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.boosts DROP CONSTRAINT IF EXISTS post_id_fk CASCADE;
ALTER TABLE public.boosts ADD CONSTRAINT post_id_fk FOREIGN KEY (post_id)
REFERENCES public.posts (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --

-- object: user_id_fk | type: CONSTRAINT --
-- ALTER TABLE public.boosts DROP CONSTRAINT IF EXISTS user_id_fk CASCADE;
ALTER TABLE public.boosts ADD CONSTRAINT user_id_fk FOREIGN KEY (user_id)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE NO ACTION;
-- ddl-end --


