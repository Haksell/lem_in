# TODO: rename lem-in
NAME := lem_in

PATH_SRCS := srcs
PATH_OBJS := objs

PATH_LIBFT := libft
LIBFT := $(PATH_LIBFT)/libft.a
LIBFT_REPO := git@github.com:Haksell/libft.git

RESET := \033[0m
RED := \033[1m\033[31m
GREEN := \033[1m\033[32m
BLUE := \033[1m\033[34m
PINK := \033[1m\033[35m

YEET := 1>/dev/null 2>/dev/null

# TODO: remove -g3
# TODO: update standard to the one from school (keep in sync with .clangd)
CC := clang -std=gnu17 -Wall -Wextra -Werror -O3 -g3

INCLUDES := -I. -I./$(PATH_LIBFT)
LIBRARIES := -L$(PATH_LIBFT) -lft

HEADER := lem_in.h
SRCS := $(wildcard $(PATH_SRCS)/*.c) $(wildcard $(PATH_SRCS)/*/*.c)  # TODO: explicit
OBJS := $(SRCS:$(PATH_SRCS)/%.c=$(PATH_OBJS)/%.o)

define clone_repo
	@echo "$(GREEN)==> Cloning $(1)$(RESET)"
	@git clone $(2) $(1) $(YEET)
	@rm -rf $(1)/.git*
endef

define compile_target
	@echo "$(GREEN)==> Compiling $(1)$(RESET)"
	@$(MAKE) -s -C $(1) $(YEET)
endef

define remove_target
@if [ -e "$(1)" ]; then \
	rm -rf "$(1)"; \
	echo "$(RED)[X] $(1) removed.$(RESET)"; \
fi
endef

all: $(NAME)

run: all
	@./$(NAME)

valgrind: all
	@valgrind --leak-check=full --show-leak-kinds=all ./$(NAME)

$(PATH_OBJS):
	@mkdir -p $(sort $(dir $(OBJS)))

$(OBJS): $(PATH_OBJS)/%.o: $(PATH_SRCS)/%.c $(HEADER) $(LIBFT) | $(PATH_OBJS)
	@mkdir -p $(PATH_OBJS)
	@$(CC) -c $< -o $@ $(INCLUDES)
	@echo "$(BLUE)+++ $@$(RESET)"

$(NAME): $(OBJS)
	@$(CC) $(OBJS) $(LIBRARIES) -o $@
	@echo "$(PINK)$@ is compiled.$(RESET)"

$(PATH_LIBFT):
	$(call clone_repo,$(PATH_LIBFT),$(LIBFT_REPO))

$(LIBFT): | $(PATH_LIBFT)
	$(call compile_target,$(PATH_LIBFT))

clean:
	$(call remove_target,$(PATH_LIBFT))
	$(call remove_target,$(PATH_OBJS))

fclean: clean
	$(call remove_target,$(NAME))

re: fclean
	@$(MAKE) -s $(NAME)

.PHONY: all run valgrind clean fclean re