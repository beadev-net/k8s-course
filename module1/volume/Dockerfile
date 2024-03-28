FROM ubuntu:latest

# Install NFS server
RUN apt-get update && \
    apt-get install -y nfs-kernel-server

# Create a directory to be exported
RUN mkdir -p /mnt/data

# Configure NFS exports
RUN echo "/mnt/data *(rw,sync,no_subtree_check,no_root_squash)" >> /etc/exports

RUN chmod -R 777 /mnt/data

# Expose NFS ports
EXPOSE 111/udp 111/tcp 2049/tcp 2049/udp

# Start NFS server
CMD ["/usr/sbin/rpcbind", "-f", "-w", "&&", "/usr/sbin/exportfs", "-r", "&&", "/usr/sbin/service", "nfs-kernel-server", "start"]
