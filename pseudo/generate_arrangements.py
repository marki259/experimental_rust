def generate_arrangements(n, k, current_arrangement, arrangements):
    if len(current_arrangement) == n:
        if sum(current_arrangement) == k:
            arrangements.append(current_arrangement)
        return
    if n - len(current_arrangement) < k - sum(current_arrangement):
        return
    generate_arrangements(n, k, current_arrangement + [0], arrangements)
    generate_arrangements(n, k, current_arrangement + [1], arrangements)

# Example usage
n = 20
k = 10
arrangements = []
generate_arrangements(n, k, [], arrangements)
print(arrangements)
