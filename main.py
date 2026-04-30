from agents.topic_agent import generate_topics
from agents.content_agent import generate_content

def main():
    topics = generate_topics()
    for topic in topics:
        content = generate_content(topic)
        print(f"\nTopic: {topic}")
        print(f"Content: {content}")

if __name__ == "__main__":
    main()
